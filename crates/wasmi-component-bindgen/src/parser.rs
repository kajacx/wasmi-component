use heck::{ToSnakeCase, ToUpperCamelCase};
use std::fmt::Write;
use wit_parser::{Interface, Type, UnresolvedPackage};
use wit_parser::{World, WorldItem};

pub struct Parser {
    pkg: UnresolvedPackage,
}

impl Parser {
    pub fn new(pkg: UnresolvedPackage) -> Self {
        Self { pkg }
    }

    pub fn parse_wit(&self) -> String {
        let mut output = String::new();

        self.pkg
            .worlds
            .iter()
            .for_each(|(_, world)| self.generate_world(world, &mut output));

        output
    }

    fn generate_world(&self, world: &World, output: &mut String) {
        let exports_name = format!("{}Exports", world.name.to_upper_camel_case());

        writeln!(output, "pub struct {exports_name} {{").unwrap();
        world.exports.values().for_each(|value| match value {
            WorldItem::Function(_func) => {
                todo!("Exported function directly in the world.");
            }
            WorldItem::Interface { id, .. } => {
                self.write_interface_export_fields(self.pkg.interfaces.get(*id).unwrap(), output);
            }
            _ => {}
        });
        writeln!(output, "}}").unwrap();

        let args =
            "store_ctx: &mut ::wasmi_component::Store, component: &::wasmi_component::Component";
        writeln!(
            output,
            "pub fn instantiate_{}_world({args}) -> {exports_name}",
            world.name.to_snake_case()
        )
        .unwrap();
    }

    fn write_interface_export_fields(&self, interface: &Interface, output: &mut String) {
        interface.functions.values().for_each(|func| {
            let field_name = format!(
                "{}_{}",
                interface.name.as_ref().unwrap().to_snake_case(),
                func.name.to_snake_case()
            );

            let mut param_types: Vec<_> = func
                .params
                .iter()
                .map(|param| self.get_type_name(&param.ty))
                .collect();

            let params = if param_types.len() == 1 {
                param_types.remove(0)
            } else {
                format!("({})", param_types.join(", "))
            };

            let result = func
                .result
                .map_or_else(|| "()".to_string(), |res| self.get_type_name(&res));

            writeln!(
                output,
                "    pub {field_name}: ::wasmi_component::TypedFunc<{params}, {result}>,"
            )
            .unwrap();
        });
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
            Type::Id(_) => todo!(),
        }
    }
}
