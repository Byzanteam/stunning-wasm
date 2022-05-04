#![cfg_attr(not(test), no_main)]

use jet_programmable_rust_binding::{
    outputs::Outputs,
    program,
    value_presenter::{
        field_type::FieldType, field_value::NumericFieldValue, literal::LiteralValuePresenter,
    },
    value_presenter::{value::number::Number, ValuePresenter},
};

fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {
    let first = inputs.get(0).unwrap();
    let second = inputs.get(1).unwrap();

    let sum: Number = add(extract_number(first), extract_number(second));

    Outputs::build(vec![ValuePresenter::Literal(
        LiteralValuePresenter::NumericField(NumericFieldValue::Value(sum)),
    )])
}

fn extract_number(value_presenter: &ValuePresenter) -> Number {
    match value_presenter {
        ValuePresenter::Literal(LiteralValuePresenter::NumericField(NumericFieldValue::Value(
            number,
        ))) => number.clone(),
        ValuePresenter::Literal(LiteralValuePresenter::NumericField(NumericFieldValue::Nil)) => {
            Number::Integer(0)
        }
        _ => unreachable!("Unexpected value presenter: {:?}", value_presenter),
    }
}

fn add(a: Number, b: Number) -> Number {
    match (a, b) {
        (Number::Integer(first), Number::Integer(second)) => Number::Integer(first + second),
        (Number::Integer(first), Number::Float(second)) => Number::Float(first as f64 + second),
        (Number::Float(first), Number::Integer(second)) => Number::Float(first + second as f64),
        (Number::Float(first), Number::Float(second)) => Number::Float(first + second),
    }
}

program!(
    entrypoint,
    vec![FieldType::NumericField, FieldType::NumericField]
);
