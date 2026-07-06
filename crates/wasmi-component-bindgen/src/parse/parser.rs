use heck::{ToSnakeCase, ToUpperCamelCase};
use wit_parser::{
    Function, Handle, Interface, Resolve, Type, TypeDefKind, TypeId, World, WorldItem, WorldKey,
};

use crate::parse::{Func, Param, ParsedWorld};

pub struct Parser {
    resolve: Resolve,
}

impl Parser {
    pub fn new(resolve: Resolve) -> Self {
        Self { resolve }
    }

    pub fn parse_wit(&self) -> Vec<ParsedWorld> {
        self.resolve
            .worlds
            .iter()
            .map(|(_, world)| self.parse_world(world))
            .collect()
    }

    fn parse_world(&self, world: &World) -> ParsedWorld {
        let world_name = world.name.clone();

        eprintln!(
            "world: {world_name}, {}, {}, {}",
            world.imports.len(),
            self.resolve.types.len(),
            self.resolve.interfaces.len()
        );

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
                let interface = &self.resolve.interfaces[*id];
                eprintln!("processing interface {:?}", interface.name.as_ref());

                interface
                    .functions
                    .iter()
                    .map(|(_name, func)| self.parse_function(func, key, Some(interface)))
                    .collect()
            }
            WorldItem::Type { id, .. } => {
                let ty = &self.resolve.types[*id];
                eprintln!("Found Type {:?}", ty);
                match ty.kind {
                    TypeDefKind::Resource => {
                        eprintln!("Found resource {}", key.clone().unwrap_name());
                    }
                    _ => {}
                }
                vec![]
            }
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
            (Some(iface), Some(interface_name)) => {
                let pkg = &self.resolve.packages[iface.package.unwrap()];

                let namespace = &pkg.name.namespace;
                let name = &pkg.name.name;

                let version = pkg.name.version.as_ref();
                let version = version.map(|v| format!("@{v}")).unwrap_or_default();

                Some(format!("{namespace}:{name}/{interface_name}{version}"))
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
            name: rust_snake_case(&param.name),
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
                let ty = &self.resolve.types[*id];
                eprintln!("matching {:?}", ty.kind);
                match &ty.kind {
                    TypeDefKind::List(list_ty) => format!("Vec<{}>", self.get_type_name(&list_ty)),
                    TypeDefKind::Resource => format!("TODO_Resource"),
                    TypeDefKind::Handle(Handle::Own(id)) => {
                        format!("Own<{}>", self.resource_name(*id))
                    }
                    TypeDefKind::Handle(Handle::Borrow(id)) => {
                        format!("Borrow<{}>", self.resource_name(*id))
                    }
                    TypeDefKind::Result(res) => {
                        let ok = res
                            .ok
                            .as_ref()
                            .map_or_else(|| "()".to_string(), |ok| self.get_type_name(&ok));

                        let err = res
                            .err
                            .as_ref()
                            .map_or_else(|| "()".to_string(), |err| self.get_type_name(&err));

                        format!("Result<{ok}, {err}>")
                    }
                    TypeDefKind::Variant(var) => format!("TODO_Variant"),
                    TypeDefKind::Tuple(tuple) => {
                        let mut result = String::from("(");
                        for item in &tuple.types {
                            result.push_str(&self.get_type_name(item));
                            result.push_str(", ");
                        }
                        result.push(')');
                        result
                    }
                    TypeDefKind::Option(option) => {
                        format!("Option<{}>", self.get_type_name(option))
                    }
                    _ => todo!(),
                }
            }
        }
    }

    fn resource_name(&self, id: TypeId) -> String {
        let name = self.resolve.types[id].name.as_ref().unwrap();
        format!("{}Resource", name.to_upper_camel_case())
    }
}

static KEYWORDS: [&'static str; 38] = [
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

fn rust_snake_case(name: impl AsRef<str>) -> String {
    let name = name.as_ref().to_snake_case();
    if KEYWORDS.contains(&name.as_str()) {
        format!("{name}_")
    } else {
        name
    }
}
