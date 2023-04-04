#![cfg_attr(not(test), no_main)]

use jet_programmable_rust_binding::{
    hostcalls::hostcall_set_outputs,
    outputs::Outputs,
    value_presenter::{
        error::DecodeError,
        field_type::FieldType,
        literal_list_value::{
            BooleanListFieldValue, CascaderListFieldValue, DateTimeListFieldValue,
            FileListFieldValue, MultipleLineListFieldValue, NumericListFieldValue,
            RelationListFieldValue, SingleLineListFieldValue, TableRowListFieldValue,
        },
        literal_naive_value::{
            BooleanFieldValue, CascaderFieldValue, DateTimeFieldValue, FileFieldValue,
            MultipleLineFieldValue, NumericFieldValue, RelationFieldValue, SingleLineFieldValue,
            TableRowFieldValue,
        },
        literal_value_presenter::LiteralValuePresenter,
        value::number::Number,
        ValuePresenter,
    },
};
use serde_json::Value;

#[no_mangle]
pub fn run(inputs: &str) {
    let json: Value = match serde_json::from_str(inputs) {
        Ok(json) => json,
        Err(err) => panic!("Failed to parse inputs: {}", err),
    };

    let (list_vp, index) = match extract_list_and_index(&json) {
        Ok((list_vp, index)) => (list_vp, index),
        Err(err) => panic!("Failed to extract list and index: {:?}", err),
    };

    let (item_vp, out_of_bounds) = list_at(&list_vp, index);

    let outputs = Outputs::build(vec![
        item_vp,
        ValuePresenter::Literal(LiteralValuePresenter::BooleanField(
            BooleanFieldValue::Value(out_of_bounds),
        )),
    ]);

    let str = outputs.to_json().to_string();

    unsafe {
        hostcall_set_outputs(str.as_ptr(), str.len());
    }
}

fn extract_list_and_index(json: &Value) -> Result<(ValuePresenter, i64), DecodeError> {
    let array = match json {
        Value::Array(array) => array,
        _ => panic!("Expect an array"),
    };

    if array.len() != 2 {
        return Err(DecodeError::InvalidJsonObject(json));
    }

    let list_vp = match array.get(0) {
        Some(list) => match ValuePresenter::from_json(list) {
            Ok(vp) => {
                if is_list_field_type(&vp.get_field_type()) {
                    Ok(vp)
                } else {
                    return Err(DecodeError::InvalidJsonObject(list));
                }
            }
            Err(err) => return Err(err),
        },
        None => Err(DecodeError::InvalidJsonObject(json)),
    };

    let index = match array.get(1) {
        Some(index) => match ValuePresenter::from_json(index) {
            Ok(ValuePresenter::Literal(LiteralValuePresenter::NumericField(
                NumericFieldValue::Value(Number::Integer(index)),
            ))) => Ok(index),
            Ok(_) => Err(DecodeError::InvalidJsonObject(index)),
            Err(err) => return Err(err),
        },
        None => Err(DecodeError::InvalidJsonObject(json)),
    };

    Ok((list_vp?, index?))
}

fn list_at(list_vp: &ValuePresenter, index: i64) -> (ValuePresenter, bool) {
    match list_vp {
        ValuePresenter::Literal(LiteralValuePresenter::BooleanListField(
            BooleanListFieldValue::Value(values),
        )) => match values.get(normalize_index(index, values.len())) {
            Some(value) => (
                ValuePresenter::Literal(LiteralValuePresenter::BooleanField(value.to_owned())),
                false,
            ),
            None => (
                ValuePresenter::Literal(LiteralValuePresenter::BooleanField(
                    BooleanFieldValue::Nil,
                )),
                true,
            ),
        },
        ValuePresenter::Literal(LiteralValuePresenter::BooleanListField(
            BooleanListFieldValue::Nil,
        )) => (
            ValuePresenter::Literal(LiteralValuePresenter::BooleanField(BooleanFieldValue::Nil)),
            true,
        ),
        ValuePresenter::Literal(LiteralValuePresenter::CascaderListField(
            CascaderListFieldValue::Value(values),
        )) => match values.get(normalize_index(index, values.len())) {
            Some(value) => (
                ValuePresenter::Literal(LiteralValuePresenter::CascaderField(value.to_owned())),
                false,
            ),
            None => (
                ValuePresenter::Literal(LiteralValuePresenter::CascaderField(
                    CascaderFieldValue::Nil,
                )),
                true,
            ),
        },
        ValuePresenter::Literal(LiteralValuePresenter::CascaderListField(
            CascaderListFieldValue::Nil,
        )) => (
            ValuePresenter::Literal(LiteralValuePresenter::CascaderField(
                CascaderFieldValue::Nil,
            )),
            true,
        ),
        ValuePresenter::Literal(LiteralValuePresenter::DateTimeListField(
            DateTimeListFieldValue::Value(values),
        )) => match values.get(normalize_index(index, values.len())) {
            Some(value) => (
                ValuePresenter::Literal(LiteralValuePresenter::DateTimeField(value.to_owned())),
                false,
            ),
            None => (
                ValuePresenter::Literal(LiteralValuePresenter::DateTimeField(
                    DateTimeFieldValue::Nil,
                )),
                true,
            ),
        },
        ValuePresenter::Literal(LiteralValuePresenter::DateTimeListField(
            DateTimeListFieldValue::Nil,
        )) => (
            ValuePresenter::Literal(LiteralValuePresenter::DateTimeField(
                DateTimeFieldValue::Nil,
            )),
            true,
        ),

        ValuePresenter::Literal(LiteralValuePresenter::FileListField(
            FileListFieldValue::Value(values),
        )) => match values.get(normalize_index(index, values.len())) {
            Some(value) => (
                ValuePresenter::Literal(LiteralValuePresenter::FileField(value.to_owned())),
                false,
            ),
            None => (
                ValuePresenter::Literal(LiteralValuePresenter::FileField(FileFieldValue::Nil)),
                true,
            ),
        },
        ValuePresenter::Literal(LiteralValuePresenter::FileListField(FileListFieldValue::Nil)) => (
            ValuePresenter::Literal(LiteralValuePresenter::FileField(FileFieldValue::Nil)),
            true,
        ),

        ValuePresenter::Literal(LiteralValuePresenter::MultipleLineListField(
            MultipleLineListFieldValue::Value(values),
        )) => match values.get(normalize_index(index, values.len())) {
            Some(value) => (
                ValuePresenter::Literal(LiteralValuePresenter::MultipleLineField(value.to_owned())),
                false,
            ),
            None => (
                ValuePresenter::Literal(LiteralValuePresenter::MultipleLineField(
                    MultipleLineFieldValue::Nil,
                )),
                true,
            ),
        },
        ValuePresenter::Literal(LiteralValuePresenter::MultipleLineListField(
            MultipleLineListFieldValue::Nil,
        )) => (
            ValuePresenter::Literal(LiteralValuePresenter::MultipleLineField(
                MultipleLineFieldValue::Nil,
            )),
            true,
        ),

        ValuePresenter::Literal(LiteralValuePresenter::NumericListField(
            NumericListFieldValue::Value(values),
        )) => match values.get(normalize_index(index, values.len())) {
            Some(value) => (
                ValuePresenter::Literal(LiteralValuePresenter::NumericField(value.to_owned())),
                false,
            ),
            None => (
                ValuePresenter::Literal(LiteralValuePresenter::NumericField(
                    NumericFieldValue::Nil,
                )),
                true,
            ),
        },
        ValuePresenter::Literal(LiteralValuePresenter::NumericListField(
            NumericListFieldValue::Nil,
        )) => (
            ValuePresenter::Literal(LiteralValuePresenter::NumericField(NumericFieldValue::Nil)),
            true,
        ),

        ValuePresenter::Literal(LiteralValuePresenter::RelationListField(
            RelationListFieldValue::Value(values),
        )) => match values.get(normalize_index(index, values.len())) {
            Some(value) => (
                ValuePresenter::Literal(LiteralValuePresenter::RelationField(value.to_owned())),
                false,
            ),
            None => (
                ValuePresenter::Literal(LiteralValuePresenter::RelationField(
                    RelationFieldValue::Nil,
                )),
                true,
            ),
        },
        ValuePresenter::Literal(LiteralValuePresenter::RelationListField(
            RelationListFieldValue::Nil,
        )) => (
            ValuePresenter::Literal(LiteralValuePresenter::RelationField(
                RelationFieldValue::Nil,
            )),
            true,
        ),

        ValuePresenter::Literal(LiteralValuePresenter::SingleLineListField(
            SingleLineListFieldValue::Value(values),
        )) => match values.get(normalize_index(index, values.len())) {
            Some(value) => (
                ValuePresenter::Literal(LiteralValuePresenter::SingleLineField(value.to_owned())),
                false,
            ),
            None => (
                ValuePresenter::Literal(LiteralValuePresenter::SingleLineField(
                    SingleLineFieldValue::Nil,
                )),
                true,
            ),
        },
        ValuePresenter::Literal(LiteralValuePresenter::SingleLineListField(
            SingleLineListFieldValue::Nil,
        )) => (
            ValuePresenter::Literal(LiteralValuePresenter::SingleLineField(
                SingleLineFieldValue::Nil,
            )),
            true,
        ),

        ValuePresenter::Literal(LiteralValuePresenter::TableRowListField(
            TableRowListFieldValue::Value(values),
        )) => match values.get(normalize_index(index, values.len())) {
            Some(value) => (
                ValuePresenter::Literal(LiteralValuePresenter::TableRowField(value.to_owned())),
                false,
            ),
            None => (
                ValuePresenter::Literal(LiteralValuePresenter::TableRowField(
                    TableRowFieldValue::Nil,
                )),
                true,
            ),
        },
        ValuePresenter::Literal(LiteralValuePresenter::TableRowListField(
            TableRowListFieldValue::Nil,
        )) => (
            ValuePresenter::Literal(LiteralValuePresenter::TableRowField(
                TableRowFieldValue::Nil,
            )),
            true,
        ),

        _ => unreachable!("Expect a list"),
    }
}

fn is_list_field_type(field_type: &FieldType) -> bool {
    matches!(
        field_type,
        FieldType::BooleanListField
            | FieldType::CascaderListField
            | FieldType::DateTimeListField
            | FieldType::FileListField
            | FieldType::MultipleLineListField
            | FieldType::NumericListField
            | FieldType::RelationListField
            | FieldType::SingleLineListField
            | FieldType::TableRowListField
    )
}

fn normalize_index(index: i64, len: usize) -> usize {
    if index < 0 {
        (index + len as i64) as usize
    } else {
        index as usize
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_extract_list_and_index() {
        {
            let json = json!([{
                "type": "literal",
                "field_type": "boolean_list_field",
                "value": [true, false, null]
            },
            {
                "type": "literal",
                "field_type": "numeric_field",
                "value": 0_i64
            }]);

            let result = extract_list_and_index(&json);

            let (list_vp, index) = result.unwrap();

            assert!(matches!(
                list_vp,
                ValuePresenter::Literal(LiteralValuePresenter::BooleanListField(
                    BooleanListFieldValue::Value(_values)
                ))
            ));

            assert_eq!(index, 0);
        }

        // negative
        {
            let json = json!([{
                "type": "literal",
                "field_type": "boolean_list_field",
                "value": [true, false, null]
            },
            {
                "type": "literal",
                "field_type": "numeric_field",
                "value": -1_i64
            }]);

            let result = extract_list_and_index(&json);

            let (list_vp, index) = result.unwrap();

            assert!(matches!(
                list_vp,
                ValuePresenter::Literal(LiteralValuePresenter::BooleanListField(
                    BooleanListFieldValue::Value(_values)
                ))
            ));

            assert_eq!(index, -1);
        }

        {
            let json = json!([{
                "type": "literal",
                "field_type": "boolean_field",
                "value": true
            },
            {
                "type": "literal",
                "field_type": "numeric_field",
                "value": 0_i64
            }]);

            let result = extract_list_and_index(&json);

            assert!(result.is_err());
        }

        {
            let json = json!([{
                "type": "literal",
                "field_type": "boolean_list_field",
                "value": [true, false, null]
            },
            {
                "type": "literal",
                "field_type": "numeric_field",
                "value": 0.1_f64
            }]);

            let result = extract_list_and_index(&json);

            assert!(result.is_err());
        }
    }

    #[test]
    fn test_list_at() {
        let list_vp = ValuePresenter::Literal(LiteralValuePresenter::BooleanListField(
            BooleanListFieldValue::Value(vec![
                BooleanFieldValue::Value(true),
                BooleanFieldValue::Value(false),
                BooleanFieldValue::Nil,
            ]),
        ));

        {
            let (item_vp, out_of_bounds) = list_at(&list_vp, 0);

            assert!(matches!(
                item_vp,
                ValuePresenter::Literal(LiteralValuePresenter::BooleanField(
                    BooleanFieldValue::Value(true)
                ))
            ));

            assert!(!out_of_bounds);
        }

        {
            let (item_vp, out_of_bounds) = list_at(&list_vp, 1);

            assert!(matches!(
                item_vp,
                ValuePresenter::Literal(LiteralValuePresenter::BooleanField(
                    BooleanFieldValue::Value(false)
                ))
            ));

            assert!(!out_of_bounds);
        }

        {
            let (item_vp, out_of_bounds) = list_at(&list_vp, 2);

            assert!(matches!(
                item_vp,
                ValuePresenter::Literal(LiteralValuePresenter::BooleanField(
                    BooleanFieldValue::Nil
                ))
            ));

            assert!(!out_of_bounds);
        }

        {
            let (item_vp, out_of_bounds) = list_at(&list_vp, -2);

            assert!(matches!(
                item_vp,
                ValuePresenter::Literal(LiteralValuePresenter::BooleanField(
                    BooleanFieldValue::Value(false)
                ))
            ));

            assert!(!out_of_bounds);
        }

        {
            let (item_vp, out_of_bounds) = list_at(&list_vp, 3);

            assert!(matches!(
                item_vp,
                ValuePresenter::Literal(LiteralValuePresenter::BooleanField(
                    BooleanFieldValue::Nil
                ))
            ));

            assert!(out_of_bounds);
        }

        {
            let (item_vp, out_of_bounds) = list_at(&list_vp, -4);

            assert!(matches!(
                item_vp,
                ValuePresenter::Literal(LiteralValuePresenter::BooleanField(
                    BooleanFieldValue::Nil
                ))
            ));

            assert!(out_of_bounds);
        }
    }
}
