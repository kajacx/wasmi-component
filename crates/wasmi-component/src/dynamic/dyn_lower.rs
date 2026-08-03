use wasmi_component_parser::ValueType;

use crate::lib_structs::{LowerBytesWriter, LowerWriter, MemoryAccess};
use crate::{ConvertResult, DynValue, Lower};

pub(crate) fn dyn_lower(
    ty: &ValueType,
    value: &DynValue,
    writer: &mut impl LowerWriter,
) -> ConvertResult<()> {
    match value {
        DynValue::S8(value) => (*value).lower(writer),
        DynValue::S16(value) => (*value).lower(writer),
        DynValue::S32(value) => (*value).lower(writer),
        DynValue::S64(value) => (*value).lower(writer),

        DynValue::U8(value) => (*value).lower(writer),
        DynValue::U16(value) => (*value).lower(writer),
        DynValue::U32(value) => (*value).lower(writer),
        DynValue::U64(value) => (*value).lower(writer),

        DynValue::F32(value) => (*value).lower(writer),
        DynValue::F64(value) => (*value).lower(writer),

        DynValue::Bool(value) => (*value).lower(writer),
        DynValue::Char(value) => (*value).lower(writer),
        DynValue::String(value) => (*value).lower(writer),

        DynValue::Option(value) => {
            let inner_ty = ty.as_option().expect("type was checked before");
            match value {
                None => writer.write_dyn_variant(ty, 0, None),
                Some(value) => writer.write_dyn_variant(ty, 1, Some((inner_ty, value))),
            }
        }
        DynValue::Result(value) => {
            let (ok_ty, err_ty) = ty.as_result().expect("type was checked before");
            match value {
                Ok(value) => writer.write_dyn_variant(ty, 0, Some((ok_ty, value))),
                Err(value) => writer.write_dyn_variant(ty, 1, Some((err_ty, value))),
            }
        }
        DynValue::Tuple(fields) => {
            let align = ty.byte_align();
            let types = ty.as_tuple().expect("type was checked before");

            for (ty, value) in types.iter().zip(fields.iter()) {
                writer.write_dyn_field(ty, value, align)?;
            }

            Ok(())
        }
        DynValue::List(contents) => {
            let inner_ty = ty.list_type().expect("type was checked before");
            let memory = writer.memory();

            let len = inner_ty.byte_size() * contents.len();
            let start = memory.allocate(len, inner_ty.byte_align())?;

            let mut list_writer = LowerBytesWriter::new(memory, start);
            for item in contents.iter() {
                dyn_lower(inner_ty, item, &mut list_writer)?;
            }

            (start as u32, contents.len() as u32).lower(writer)
        }

        DynValue::Record { fields } => {
            let align = ty.byte_align();
            let (_name, fields_ty) = ty.as_record().expect("type was checked before");

            for (name, ty) in fields_ty.iter() {
                let value = fields.get_field(&name).expect("type was checked before");
                writer.write_dyn_field(ty, value, align)?;
            }

            Ok(())
        }
        DynValue::Variant { determinant, value } => {
            let (_name, cases) = ty.as_variant().expect("type was checked before");
            let index = cases
                .iter()
                .position(|(name, _)| name == determinant)
                .expect("type was checked before");

            if let Some(value) = value {
                writer.write_dyn_variant(
                    ty,
                    index,
                    Some((
                        cases[index].1.as_ref().expect("type was checked before"),
                        value,
                    )),
                )
            } else {
                writer.write_dyn_variant(ty, index, None)
            }
        }
    }
}
