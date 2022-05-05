#![cfg_attr(not(test), no_main)]

use jet_programmable_rust_binding::{
    outputs::Outputs, program, value_presenter::field_type::FieldType,
    value_presenter::ValuePresenter,
};

fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {
    Outputs::build(inputs)
}

program!(
    entrypoint,
    vec![
        FieldType::DateTimeField,
        FieldType::NumericField,
        FieldType::RadioButtonField,
    ]
);
