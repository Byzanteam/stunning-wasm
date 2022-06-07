#![cfg_attr(not(test), no_main)]

use jet_programmable_rust_binding::{
    outputs::Outputs,
    program,
    value_presenter::{
        field_type::FieldType,
        literal_naive_value::{CascaderFieldValue, RelationFieldValue, TableRowFieldValue},
        literal_value_presenter::LiteralValuePresenter,
        value::cascader_value::CascaderValue,
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
    let table = inputs
        .get(1)
        .unwrap()
        .as_literal()
        .unwrap()
        .as_relation_field_value()
        .unwrap();

    let cascader_value = convert(value, table);

    Outputs::build(vec![ValuePresenter::Literal(
        LiteralValuePresenter::CascaderField(cascader_value),
    )])
}

fn convert(value: &TableRowFieldValue, table: &RelationFieldValue) -> CascaderFieldValue {
    match value {
        TableRowFieldValue::Value(row_uuid) => match table {
            RelationFieldValue::Value(relation_value) => CascaderFieldValue::Value(
                CascaderValue::new(relation_value.resource_uuid.clone(), row_uuid.clone()),
            ),
            RelationFieldValue::Nil => CascaderFieldValue::Nil,
        },
        TableRowFieldValue::Nil => CascaderFieldValue::Nil,
    }
}

program!(
    entrypoint,
    vec![FieldType::TableRowField, FieldType::RelationField]
);

#[cfg(test)]
mod tests {
    use jet_programmable_rust_binding::value_presenter::{
        literal_naive_value::{CascaderFieldValue, RelationFieldValue, TableRowFieldValue},
        value::{
            cascader_value::CascaderValue,
            relation_value::{RelationValue, ResourceType},
            uuid::Uuid,
        },
    };

    use super::*;

    #[test]
    fn test_converts() {
        let row_uuid = Uuid::new("67e55044-10b1-426f-9247-000000000000").unwrap();
        let table_uuid = Uuid::new("67e55044-10b1-426f-9247-000000000001").unwrap();

        let value = TableRowFieldValue::Value(row_uuid.clone());
        let table = RelationFieldValue::Value(RelationValue::new(
            ResourceType::DatabaseTable,
            table_uuid.clone(),
        ));

        let expected = CascaderFieldValue::Value(CascaderValue::new(table_uuid, row_uuid));

        assert_eq!(convert(&value, &table), expected);
    }

    #[test]
    fn test_converts_nil_value() {
        let table_uuid = Uuid::new("67e55044-10b1-426f-9247-000000000001").unwrap();

        let value = TableRowFieldValue::Nil;
        let table =
            RelationFieldValue::Value(RelationValue::new(ResourceType::DatabaseTable, table_uuid));

        let expected = CascaderFieldValue::Nil;

        assert_eq!(convert(&value, &table), expected);
    }

    #[test]
    fn test_converts_nil_table() {
        let row_uuid = Uuid::new("67e55044-10b1-426f-9247-000000000000").unwrap();

        let value = TableRowFieldValue::Value(row_uuid);
        let table = RelationFieldValue::Nil;

        let expected = CascaderFieldValue::Nil;

        assert_eq!(convert(&value, &table), expected);
    }
}
