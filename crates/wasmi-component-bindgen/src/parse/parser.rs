use heck::{ToSnakeCase, ToUpperCamelCase};
use wit_parser::{
    Docs, Function, Interface, Resolve, Result_, Span, Tuple, Type, TypeDef, TypeDefKind, World,
    WorldItem, WorldKey,
};

use crate::parse::{Func, LowerArg, Param, ParamType, ParsedWit, ParsedWorld};

pub struct Parser {
    resolve: Resolve,
}

impl Parser {
    pub fn new(resolve: Resolve) -> Self {
        Self { resolve }
    }

    pub fn parse_wit(&self) -> ParsedWit {
        let types = self
            .resolve
            .types
            .iter()
            .filter_map(|(_, ty)| self.write_type(ty))
            .collect();

        let worlds = self
            .resolve
            .worlds
            .iter()
            .map(|(_, world)| self.parse_world(world))
            .collect();

        ParsedWit { types, worlds }
    }

    fn write_type(&self, ty: &TypeDef) -> Option<String> {
        match &ty.kind {
            TypeDefKind::Record(record) => {
                let mut output = String::from(
                    "#[allow(unused)]\n#[derive(Debug, Clone, PartialEq, PartialOrd, ComponentValue)]\n",
                );

                output.push_str("pub struct ");
                output.push_str(&ty.name.as_ref().unwrap().to_upper_camel_case());
                output.push_str(" {\n");

                record.fields.iter().for_each(|field| {
                    output.push_str("pub ");
                    output.push_str(&rust_snake_case(&field.name));
                    output.push_str(": ");
                    output.push_str(&self.parse_type(field.ty).canon);
                    output.push_str(",\n");
                });

                output.push_str("}\n");
                Some(output)
            }
            TypeDefKind::Variant(var) => {
                let mut output = String::from(
                    "#[allow(unused)]\n#[derive(Debug, Clone, PartialEq, PartialOrd, ComponentValue)]\n",
                );

                output.push_str("pub enum ");
                output.push_str(&ty.name.as_ref().unwrap().to_upper_camel_case());
                output.push_str(" {\n");

                var.cases.iter().for_each(|case| {
                    output.push_str(&case.name.to_upper_camel_case());
                    if let Some(ty) = case.ty {
                        output.push('(');
                        output.push_str(&self.parse_type(ty).canon);
                        output.push(')');
                    }
                    output.push_str(",\n");
                });

                output.push_str("}\n");
                Some(output)
            }
            _ => None,
        }
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

        ParsedWorld::new(world_name, imports, exports)
    }

    fn parse_world_item(&self, key: &WorldKey, item: &WorldItem) -> Vec<Func> {
        match item {
            WorldItem::Function(func) => vec![self.parse_function(func, key, None)],
            WorldItem::Interface { id, .. } => {
                let interface = &self.resolve.interfaces[*id];

                let mut result: Vec<_> = interface
                    .functions
                    .iter()
                    .map(|(_name, func)| self.parse_function(func, key, Some(interface)))
                    .collect();

                interface.types.iter().for_each(|(name, id)| {
                    let ty = &self.resolve.types[*id].kind;
                    if matches!(ty, TypeDefKind::Resource) {
                        let drop = Function {
                            docs: Docs::default(),
                            kind: wit_parser::FunctionKind::Freestanding,
                            name: format!("[resource-drop]{name}"),
                            params: vec![wit_parser::Param {
                                name: "index".to_string(),
                                ty: Type::S32,
                                span: Span::default(),
                            }],
                            result: None,
                            stability: wit_parser::Stability::Unknown,
                            span: Span::default(),
                        };
                        result.push(self.parse_function(&drop, key, Some(interface)));
                    }
                });

                result
            }
            WorldItem::Type { .. } => {
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
                let version = self.interface_version(iface);

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

        let result = func
            .result
            .map_or_else(|| ParamType::from_simple("()"), |ty| self.parse_type(ty));

        Func::new(module_name, func.name.clone(), params, result)
    }

    fn interface_version(&self, interface: &Interface) -> String {
        let package = &self.resolve.packages[interface.package.unwrap()];
        let version = package.name.version.as_ref().map(|v| v.to_string());

        if version == Some("0.2.6".to_string()) {
            "@0.2.0".to_string() // TODO: sem ver hack
        } else if let Some(ver) = version {
            format!("@{ver}")
        } else {
            "".to_string()
        }
    }

    fn parse_function_param(&self, param: &wit_parser::Param) -> Param {
        Param {
            name: rust_snake_case(&param.name),
            ty: self.parse_type(param.ty),
        }
    }

    fn parse_type(&self, ty: Type) -> ParamType {
        match ty {
            Type::Bool => ParamType::from_simple("bool"),
            Type::Char => ParamType::from_simple("char"),
            Type::S8 => ParamType::from_simple("i8"),
            Type::S16 => ParamType::from_simple("i16"),
            Type::S32 => ParamType::from_simple("i32"),
            Type::S64 => ParamType::from_simple("i64"),
            Type::U8 => ParamType::from_simple("u8"),
            Type::U16 => ParamType::from_simple("u16"),
            Type::U32 => ParamType::from_simple("u32"),
            Type::U64 => ParamType::from_simple("u64"),
            Type::F32 => ParamType::from_simple("f32"),
            Type::F64 => ParamType::from_simple("f64"),
            Type::String => ParamType {
                canon: "String".to_string(),
                lower: LowerArg::LowerValue,
                lift: "&str".to_string(),
            },
            Type::ErrorContext => todo!(),
            Type::Id(id) => {
                let ty = &self.resolve.types[id];
                match &ty.kind {
                    TypeDefKind::Option(ty) => self.prase_type_option(*ty),
                    TypeDefKind::Result(res) => self.parse_type_result(res),
                    TypeDefKind::Tuple(tuple) => self.parse_type_tuple(tuple),
                    TypeDefKind::List(ty) => {
                        let ty = self.parse_type(*ty);
                        ParamType {
                            canon: format!("Vec<{}>", ty.canon),
                            lower: LowerArg::LowerValue,
                            lift: format!("ListAccessor<{}>", ty.canon),
                        }
                    }
                    TypeDefKind::Record(_) => {
                        ParamType::from_simple(&ty.name.as_ref().unwrap().to_upper_camel_case())
                    }
                    TypeDefKind::Variant(_) => {
                        ParamType::from_simple(&ty.name.as_ref().unwrap().to_upper_camel_case())
                    }
                    TypeDefKind::Handle(_) => ParamType::from_simple("i32"),
                    _ => todo!(),
                }
            }
        }
    }

    fn prase_type_option(&self, ty: Type) -> ParamType {
        let ty = self.parse_type(ty);
        let lower = if let LowerArg::Specific(specific) = ty.lower {
            LowerArg::Specific(format!("Option<{}>", specific))
        } else {
            LowerArg::LowerValue
        };
        ParamType {
            canon: format!("Option<{}>", ty.canon),
            lower,
            lift: format!("Option<{}>", ty.lift),
        }
    }

    fn parse_type_result(&self, ty: &Result_) -> ParamType {
        let ok = ty
            .ok
            .map_or_else(|| ParamType::from_simple("()"), |ty| self.parse_type(ty));

        let err = ty
            .err
            .map_or_else(|| ParamType::from_simple("()"), |ty| self.parse_type(ty));

        let lower = if let (LowerArg::Specific(ok), LowerArg::Specific(err)) = (ok.lower, err.lower)
        {
            LowerArg::Specific(format!("Result<{}, {}>", ok, err))
        } else {
            LowerArg::LowerValue
        };

        ParamType {
            canon: format!("Result<{}, {}>", ok.canon, err.canon),
            lower,
            lift: format!("Result<{}, {}>", ok.lift, err.lift),
        }
    }

    fn parse_type_tuple(&self, tuple: &Tuple) -> ParamType {
        let types: Vec<_> = tuple.types.iter().map(|ty| self.parse_type(*ty)).collect();

        let canon = format!(
            "({})",
            types
                .iter()
                .map(|ty| format!("{}, ", ty.canon))
                .collect::<String>()
        );

        let lower = types
            .iter()
            .map(|ty| ty.lower.specific())
            .try_fold(String::new(), |mut accu, item| {
                accu.push_str(item?);
                accu.push_str(", ");
                Some(accu)
            })
            .map_or(LowerArg::LowerValue, |types| {
                LowerArg::Specific(format!("({types})"))
            });

        let lift = format!(
            "({})",
            types
                .iter()
                .map(|ty| format!("{}, ", ty.lift))
                .collect::<String>()
        );

        ParamType { canon, lower, lift }
    }
}

static KEYWORDS: [&'static str; 38] = [
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

fn rust_snake_case(name: impl AsRef<str>) -> String {
    let mut name = name.as_ref().to_snake_case();
    if KEYWORDS.contains(&name.as_str()) {
        name.push('_');
    }
    name
}
