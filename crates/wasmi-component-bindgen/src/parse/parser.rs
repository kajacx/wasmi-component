use heck::{ToSnakeCase, ToUpperCamelCase};
use wit_parser::{
    Docs, Function, Handle, Interface, Resolve, Span, Type, TypeDef, TypeDefKind, TypeId, World,
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
            .flat_map(|(_, ty)| self.parse_type(ty))
            .collect();

        let worlds = self
            .resolve
            .worlds
            .iter()
            .map(|(_, world)| self.parse_world(world))
            .collect();

        ParsedWit { types, worlds }
    }

    fn parse_type(&self, ty: &TypeDef) -> Vec<String> {
        match &ty.kind {
            // TypeDefKind::Variant(var) => {
            //     let mut output = String::from("#[allow(unused)]\n#[derive(Debug)]\n");

            //     output.push_str("pub enum ");
            //     output.push_str(&ty.name.as_ref().unwrap().to_upper_camel_case());
            //     output.push_str(" {\n");

            //     var.cases.iter().for_each(|case| {
            //         output.push_str(&case.name.to_upper_camel_case());
            //         if let Some(ty) = case.ty {
            //             output.push('(');
            //             output.push_str(&self.get_type_name(ty));
            //             output.push(')');
            //         }
            //         output.push_str(",\n");
            //     });

            //     output.push_str("}\n");

            //     vec![output]
            // }
            _ => vec![],
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
                // let ty = &self.resolve.types[*id].kind;
                // if matches!(ty, TypeDefKind::Resource) {
                //     eprintln!("Resource ? found in ?");
                // }
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

        let result = self.get_param_type_option(func.result);

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
            ty: self.get_param_type(param.ty),
        }
    }

    fn get_param_type_option(&self, ty: Option<Type>) -> ParamType {
        match ty {
            Some(ty) => self.get_param_type(ty),
            None => ParamType {
                canon: "()".to_string(),
                lower: LowerArg::Specific("()".to_string()),
                lift: "()".to_string(),
            },
        }
    }

    fn get_param_type(&self, ty: Type) -> ParamType {
        return match self.pre_parse_type(ty) {
            PreParsedType::Primitive(name) => ParamType {
                canon: name.to_string(),
                lower: LowerArg::Specific(name.to_string()),
                lift: name.to_string(),
            },
            PreParsedType::String => ParamType {
                canon: "String".to_string(),
                lower: LowerArg::LowerVal,
                lift: "&str".to_string(),
            },
            PreParsedType::List(ty) => {
                let ty = self.get_param_type(ty);
                ParamType {
                    canon: format!("Vec<{}>", ty.canon),
                    lower: LowerArg::LowerVal,
                    lift: format!("ListAccessor<{}>", ty.canon),
                }
            }
            PreParsedType::Option(ty) => {
                let ty = self.get_param_type(ty);
                let lower = if let LowerArg::Specific(specific) = ty.lower {
                    LowerArg::Specific(format!("Option<{}>", specific))
                } else {
                    LowerArg::LowerVal
                };
                ParamType {
                    canon: format!("Option<{}>", ty.canon),
                    lower,
                    lift: format!("Option<{}>", ty.lift),
                }
            }
            PreParsedType::Result(ok, err) => {
                let ok = self.get_param_type_option(ok);
                let err = self.get_param_type_option(err);
                let lower = if let (LowerArg::Specific(ok), LowerArg::Specific(err)) =
                    (ok.lower, err.lower)
                {
                    LowerArg::Specific(format!("Result<{}, {}>", ok, err))
                } else {
                    LowerArg::LowerVal
                };
                ParamType {
                    canon: format!("Result<{}, {}>", ok.canon, err.canon),
                    lower,
                    lift: format!("Result<{}, {}>", ok.lift, err.lift),
                }
            }
            PreParsedType::Tuple(types) => {
                let types: Vec<_> = types.iter().map(|ty| self.get_param_type(*ty)).collect();

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
                    .map_or(LowerArg::LowerVal, |types| {
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
            PreParsedType::Own(name) => ParamType {
                canon: format!("Own<{name}>"),
                lower: LowerArg::Specific(format!("Own<{name}>")),
                lift: format!("Own<{name}>"),
            },
            PreParsedType::Borrow(name) => ParamType {
                canon: format!("Borrow<{name}>"),
                lower: LowerArg::Specific(format!("Borrow<{name}>")),
                lift: format!("Borrow<{name}>"),
            },
        };
    }

    fn pre_parse_type(&self, ty: Type) -> PreParsedType<'_> {
        match ty {
            Type::Bool => PreParsedType::Primitive("bool".to_string()),
            Type::Char => PreParsedType::Primitive("char".to_string()),
            Type::S8 => PreParsedType::Primitive("i8".to_string()),
            Type::S16 => PreParsedType::Primitive("i16".to_string()),
            Type::S32 => PreParsedType::Primitive("i32".to_string()),
            Type::S64 => PreParsedType::Primitive("i64".to_string()),
            Type::U8 => PreParsedType::Primitive("u8".to_string()),
            Type::U16 => PreParsedType::Primitive("u16".to_string()),
            Type::U32 => PreParsedType::Primitive("u32".to_string()),
            Type::U64 => PreParsedType::Primitive("u64".to_string()),
            Type::F32 => PreParsedType::Primitive("f32".to_string()),
            Type::F64 => PreParsedType::Primitive("f64".to_string()),
            Type::String => PreParsedType::String,
            Type::ErrorContext => todo!(),
            Type::Id(id) => {
                let ty = &self.resolve.types[id];
                match &ty.kind {
                    TypeDefKind::List(ty) => PreParsedType::List(*ty),
                    TypeDefKind::Handle(Handle::Own(id)) => {
                        PreParsedType::Own(self.resource_name(*id))
                    }
                    TypeDefKind::Handle(Handle::Borrow(id)) => {
                        PreParsedType::Borrow(self.resource_name(*id))
                    }
                    TypeDefKind::Result(res) => PreParsedType::Result(res.ok, res.err),
                    TypeDefKind::Variant(_) => {
                        PreParsedType::Primitive(ty.name.as_ref().unwrap().to_upper_camel_case())
                    }
                    TypeDefKind::Tuple(tuple) => PreParsedType::Tuple(&tuple.types),
                    TypeDefKind::Option(option) => PreParsedType::Option(*option),
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
    let mut name = name.as_ref().to_snake_case();
    if KEYWORDS.contains(&name.as_str()) {
        name.push('_');
    }
    name
}

enum PreParsedType<'a> {
    Primitive(String),
    String,
    List(Type),
    Option(Type),
    Result(Option<Type>, Option<Type>),
    Tuple(&'a [Type]),
    Own(String),
    Borrow(String),
}
