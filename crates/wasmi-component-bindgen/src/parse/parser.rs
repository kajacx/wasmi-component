use heck::ToSnakeCase;
use wit_parser::{
    Function, Interface, Type, TypeDefKind, UnresolvedPackage, World, WorldItem, WorldKey,
};

use crate::parse::{Func, Param, ParsedWorld};

pub struct Parser {
    pkg: UnresolvedPackage,
}

impl Parser {
    pub fn new(pkg: UnresolvedPackage) -> Self {
        Self { pkg }
    }

    pub fn parse_wit(&self) -> Vec<ParsedWorld> {
        self.pkg
            .worlds
            .iter()
            .map(|(_, world)| self.parse_world(world))
            .collect()
    }

    fn parse_world(&self, world: &World) -> ParsedWorld {
        let world_name = world.name.clone();

        let imports = world
            .imports
            .iter()
            .flat_map(|(key, item)| self.parse_world_item(key, item))
            .collect();

        let exports = world
            .exports
            .iter()
            .flat_map(|(key, item)| self.parse_world_item(key, item))
            .collect();

        ParsedWorld {
            world_name,
            imports,
            exports,
        }
    }

    fn parse_world_item(&self, key: &WorldKey, item: &WorldItem) -> Vec<Func> {
        match item {
            WorldItem::Function(func) => vec![self.parse_function(func, key, None)],
            WorldItem::Interface { id, .. } => {
                let interface = self.pkg.interfaces.get(*id).unwrap();
                interface
                    .functions
                    .iter()
                    .map(|(_name, func)| self.parse_function(func, key, Some(interface)))
                    .collect()
            }
            _ => vec![],
        }
    }

    fn parse_function(
        &self,
        func: &Function,
        key: &WorldKey,
        interface: Option<&Interface>,
    ) -> Func {
        let interface_name = interface.and_then(|iface| iface.name.as_ref());
        let module_name = match (interface, interface_name) {
            (Some(_), Some(interface_name)) => {
                let namespace = &self.pkg.name.namespace;
                let pkg_name = &self.pkg.name.name;
                let version = self
                    .pkg
                    .name
                    .version
                    .as_ref()
                    .map_or("".to_string(), |v| format!("@{v}"));

                Some(format!("{namespace}:{pkg_name}/{interface_name}{version}"))
            }
            (Some(_), None) => Some(key.clone().unwrap_name()),
            (None, _) => None,
        };

        let params: Vec<_> = func
            .params
            .iter()
            .map(|param| self.parse_function_param(param))
            .collect();

        let result = func.result.map(|ty| self.get_type_name(&ty));

        Func::new(module_name, func.name.clone(), params, result)
    }

    fn parse_function_param(&self, param: &wit_parser::Param) -> Param {
        Param {
            name: param.name.to_snake_case(),
            ty: self.get_type_name(&param.ty),
        }
    }

    fn get_type_name(&self, ty: &Type) -> String {
        match ty {
            Type::Bool => "bool".to_string(),
            Type::Char => "char".to_string(),
            Type::S8 => "i8".to_string(),
            Type::S16 => "i16".to_string(),
            Type::S32 => "i32".to_string(),
            Type::S64 => "i64".to_string(),
            Type::U8 => "u8".to_string(),
            Type::U16 => "u16".to_string(),
            Type::U32 => "u32".to_string(),
            Type::U64 => "u64".to_string(),
            Type::F32 => "f32".to_string(),
            Type::F64 => "f64".to_string(),
            Type::String => "String".to_string(),
            Type::ErrorContext => todo!(),
            Type::Id(id) => {
                let ty = self.pkg.types.get(*id).unwrap();
                match ty.kind {
                    TypeDefKind::List(list_ty) => format!("Vec<{}>", self.get_type_name(&list_ty)),
                    _ => todo!(),
                }
            }
        }
    }
}
