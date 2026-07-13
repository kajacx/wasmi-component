use crate::WasmValue;

pub fn round_up(value: usize, multiple: usize) -> usize {
    (value + multiple - 1) / multiple * multiple
}

pub fn variant_types(cases: impl IntoIterator<Item = Vec<wasmi::ValType>>) -> Vec<wasmi::ValType> {
    let mut result = vec![wasmi::ValType::I32];

    for types in cases {
        for index in 0..types.len() {
            let result_index = index + 1;
            if result_index < result.len() {
                result[result_index] =
                    WasmValue::merge_wasmi_vals(result[result_index], types[index]);
            } else {
                result.push(types[index]);
            }
        }
    }

    result
}

#[test]
fn test_round_up() {
    assert_eq!(round_up(7, 5), 10);
    assert_eq!(round_up(8, 5), 10);
    assert_eq!(round_up(9, 5), 10);
    assert_eq!(round_up(10, 5), 10);
    assert_eq!(round_up(11, 5), 15);
    assert_eq!(round_up(12, 5), 15);
}

#[test]
fn test_variant_types() {
    use wasmi::ValType::*;

    assert_eq!(variant_types([]), vec![I32,]);

    assert_eq!(
        variant_types([vec![I32, I64, F32, F64]]),
        vec![I32, I32, I64, F32, F64]
    );

    assert_eq!(
        variant_types([vec![I32, I64, F32, F64], vec![I32, I64, F32, F64]]),
        vec![I32, I32, I64, F32, F64]
    );

    assert_eq!(
        variant_types([vec![I32, I32, I64, F32], vec![F32, F64, F32, F64]]),
        vec![I32, I32, I64, I64, I64]
    );

    assert_eq!(
        variant_types([vec![I32, I32], vec![], vec![I32, F64, F64]]),
        vec![I32, I32, I64, F64]
    );
}
