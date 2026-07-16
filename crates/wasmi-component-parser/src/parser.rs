use wit_parser::{
    Docs, Function, Interface, Record, Resolve, Result_, Span, Type, TypeDef, TypeDefKind, Variant,
    World, WorldItem, WorldKey,
};

use crate::{Func, ParsedWit, ParsedWorld, ValueType};

pub struct Parser {
    pub resolve: Resolve,
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
            .filter_map(|(_, ty)| self.parse_type(ty))
            .collect();

        let worlds = self
            .resolve
            .worlds
            .iter()
            .map(|(_, world)| self.parse_world(world))
            .collect();

        ParsedWit { types, worlds }
    }

    pub fn parse_type(&self, ty: &TypeDef) -> Option<ValueType> {
        match &ty.kind {
            TypeDefKind::Record(record) => {
                Some(self.parse_record(ty.name.as_ref().expect("name of record"), &record))
            }
            TypeDefKind::Variant(variant) => {
                Some(self.parse_variant(ty.name.as_ref().expect("name of record"), &variant))
            }
            TypeDefKind::Enum(_enum) => {
                todo!("enum type")
            }
            TypeDefKind::Flags(_flags) => {
                todo!("flags type")
            }
            _ => None,
        }
    }

    pub fn parse_world(&self, world: &World) -> ParsedWorld {
        let name = world.name.clone();

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

        ParsedWorld::new(name, imports, exports)
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
                        // TODO: fix this spaghetti mess!
                        let drop = Function {
                            docs: Docs::default(),
                            kind: wit_parser::FunctionKind::Freestanding,
                            name: format!("[resource-drop]{name}"),
                            params: vec![wit_parser::Param {
                                name: "index".to_string(),
                                ty: Type::S32,
                                span: Span::default(),
                            }],
                            external_id: None,
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

                format!("{namespace}:{name}/{interface_name}{version}")
            }
            (Some(_), None) => key.clone().unwrap_name(),
            (None, _) => "".to_string(),
        };

        let params: Vec<_> = func
            .params
            .iter()
            .map(|param| (param.name.clone(), self.convert_type(param.ty)))
            .collect();

        let result = func
            .result
            .map_or_else(|| ValueType::unit(), |ty| self.convert_type(ty));

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

    fn convert_type(&self, ty: Type) -> ValueType {
        match ty {
            Type::S8 => ValueType::S8,
            Type::S16 => ValueType::S16,
            Type::S32 => ValueType::S32,
            Type::S64 => ValueType::S64,

            Type::U8 => ValueType::U8,
            Type::U16 => ValueType::U16,
            Type::U32 => ValueType::U32,
            Type::U64 => ValueType::U64,

            Type::F32 => ValueType::F32,
            Type::F64 => ValueType::F64,

            Type::Bool => ValueType::Bool,
            Type::Char => ValueType::Char,

            Type::String => ValueType::String,
            Type::ErrorContext => todo!("error context"),

            Type::Id(id) => {
                let ty = &self.resolve.types[id];
                match &ty.kind {
                    TypeDefKind::Option(ty) => ValueType::Option(Box::new(self.convert_type(*ty))),
                    TypeDefKind::Result(res) => self.parse_result(res),
                    TypeDefKind::Tuple(tuple) => ValueType::Tuple(
                        tuple
                            .types
                            .iter()
                            .map(|ty| self.convert_type(*ty))
                            .collect(),
                    ),
                    TypeDefKind::List(ty) => ValueType::List(Box::new(self.convert_type(*ty))),

                    TypeDefKind::Record(record) => {
                        self.parse_record(ty.name.as_ref().expect("name of record"), record)
                    }
                    TypeDefKind::Variant(variant) => {
                        self.parse_variant(ty.name.as_ref().expect("name of variant"), variant)
                    }

                    TypeDefKind::Handle(_) => ValueType::S32,
                    other => todo!("not yet implemented type: {other:?}"),
                }
            }
        }
    }

    fn parse_result(&self, ty: &Result_) -> ValueType {
        let ok = ty
            .ok
            .map_or_else(|| ValueType::unit(), |ty| self.convert_type(ty));

        let err = ty
            .err
            .map_or_else(|| ValueType::unit(), |ty| self.convert_type(ty));

        ValueType::Result(Box::new(ok), Box::new(err))
    }

    fn parse_record(&self, name: &str, record: &Record) -> ValueType {
        let fields = record
            .fields
            .iter()
            .map(|field| (field.name.clone(), self.convert_type(field.ty)))
            .collect();

        ValueType::Record {
            name: name.to_string(),
            fields,
        }
    }

    fn parse_variant(&self, name: &str, variant: &Variant) -> ValueType {
        let cases = variant
            .cases
            .iter()
            .map(|field| (field.name.clone(), field.ty.map(|ty| self.convert_type(ty))))
            .collect();

        ValueType::Variant {
            name: name.to_string(),
            cases,
        }
    }
}
