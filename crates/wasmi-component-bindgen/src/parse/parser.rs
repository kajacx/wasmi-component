use wit_parser::{Function, Interface, Type, UnresolvedPackage, WorldKey};
use wit_parser::{World, WorldItem};

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
        let params: Vec<_> = func
            .params
            .iter()
            .map(|param| self.parse_function_param(param))
            .collect();

        let result = func.result.map(|ty| self.get_type_name(&ty));

        Func::new(&self.pkg, func, key, interface, params, result)
    }

    fn parse_function_param(&self, param: &wit_parser::Param) -> Param {
        Param {
            name: param.name.clone(),
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
            Type::String => "WitString".to_string(),
            Type::ErrorContext => todo!(),
            Type::Id(_) => todo!(),
        }
    }
}
