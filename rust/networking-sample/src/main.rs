#![cfg_attr(not(test), no_main)]

use std::str;

use jet_programmable_rust_binding::networking::{request, NetworkingRequest};
use jet_programmable_rust_binding::{
    outputs::Outputs,
    program,
    value_presenter::ValuePresenter,
    value_presenter::{
        field_type::FieldType, literal_naive_value::SingleLineFieldValue,
        literal_value_presenter::LiteralValuePresenter,
    },
};

fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {
    let timezone = inputs
        .get(0)
        .unwrap()
        .as_literal()
        .unwrap()
        .as_single_line_field_value()
        .map_or_else(
            || "Asia/Shanghai",
            |value| match value {
                SingleLineFieldValue::Value(str) => str,
                SingleLineFieldValue::Nil => "Asia/Shanghai",
            },
        );

    let url = "http://worldtimeapi.org/api/timezone/".to_owned() + timezone;

    let request_data = NetworkingRequest::get(url, Vec::new());
    let response = request(&request_data);

    let body = match response.unwrap().body {
        Some(slice) => str::from_utf8(&slice).unwrap().to_owned(),
        None => "".to_owned(),
    };

    Outputs::build(vec![ValuePresenter::Literal(
        LiteralValuePresenter::SingleLineField(SingleLineFieldValue::Value(body)),
    )])
}

program!(entrypoint, vec![FieldType::SingleLineField]);
