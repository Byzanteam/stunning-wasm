#![no_main]
use jet_programmable_rust_binding::{
    networking::{request, NetworkingRequest},
    outputs::Outputs,
    program,
    value_presenter::ValuePresenter,
    value_presenter::{
        field_type::FieldType, literal_naive_value::SingleLineFieldValue,
        literal_value_presenter::LiteralValuePresenter,
    },
};
use serde_json::Value;

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

    let outputs = match response {
        Ok(res) => match serde_json::from_str::<Value>(&res.body.unwrap()) {
            Ok(value) => {
                if let Some(time) = value.get("datetime").unwrap().as_str() {
                    time.to_string()
                } else {
                    panic!("Invalid datetime");
                }
            }
            Err(_e) => panic!("bad response"),
        },
        Err(err) => panic!("{}",err.message),
    };
    Outputs::build(vec![ValuePresenter::Literal(
        LiteralValuePresenter::SingleLineField(SingleLineFieldValue::Value(outputs)),
    )])
}

program!(entrypoint, vec![FieldType::SingleLineField]);
