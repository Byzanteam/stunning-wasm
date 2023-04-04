#![cfg_attr(not(test), no_main)]

use jet_programmable_rust_binding::{
    outputs::Outputs,
    program,
    value_presenter::{
        field_type::FieldType,
        literal_naive_value::{RelationFieldValue, TableRowFieldValue},
        literal_value_presenter::LiteralValuePresenter,
        value::relation_value::{RelationValue, ResourceType},
        ValuePresenter,
    },
};

fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {
    let value = inputs
        .get(0)
        .unwrap()
        .as_literal()
        .unwrap()
        .as_table_row_field_value()
        .unwrap();

    let relation_value = convert(value);

    Outputs::build(vec![ValuePresenter::Literal(
        LiteralValuePresenter::RelationField(relation_value),
    )])
}

fn convert(value: &TableRowFieldValue) -> RelationFieldValue {
    match value {
        TableRowFieldValue::Value(row_uuid) => RelationFieldValue::Value(RelationValue {
            resource_type: ResourceType::DatabaseRow,
            resource_uuid: row_uuid.clone(),
        }),
        TableRowFieldValue::Nil => RelationFieldValue::Nil,
    }
}

program!(entrypoint, vec![FieldType::TableRowField]);

#[cfg(test)]
mod tests {
    use jet_programmable_rust_binding::value_presenter::{
        literal_naive_value::{RelationFieldValue, TableRowFieldValue},
        value::{
            relation_value::{RelationValue, ResourceType},
            uuid::Uuid,
        },
    };

    use super::*;

    #[test]
    fn test_converts() {
        let row_uuid = Uuid::new("67e55044-10b1-426f-9247-000000000000").unwrap();

        let value = TableRowFieldValue::Value(row_uuid.clone());

        let expected = RelationFieldValue::Value(RelationValue {
            resource_type: ResourceType::DatabaseRow,
            resource_uuid: row_uuid,
        });

        assert_eq!(convert(&value), expected);
    }

    #[test]
    fn test_converts_nil_value() {
        let value = TableRowFieldValue::Nil;

        let expected = RelationFieldValue::Nil;

        assert_eq!(convert(&value), expected);
    }
}
