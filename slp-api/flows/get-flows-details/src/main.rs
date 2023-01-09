#![cfg_attr(not(test), no_main)]

use jet_programmable_rust_binding::{
    networking::{request, NetworkingRequest},
    outputs::Outputs,
    program,
    value_presenter::{
        field_type::FieldType,
        literal_naive_value::{NumericFieldValue, SingleLineFieldValue},
        literal_value_presenter::LiteralValuePresenter,
        value::number::Number,
        ValuePresenter,
    },
};

fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {
    let domain_value_presenter = inputs.get(0).unwrap();
    let flow_id_value_presenter = inputs.get(1).unwrap();
    let authorization_token_value_presenter = inputs.get(2).unwrap();
    let domain = extract_string_url(domain_value_presenter);
    let flow_id = extract_number_id(flow_id_value_presenter);
    let authorization_token = extract_string_authorization(authorization_token_value_presenter);
    let id_str = match flow_id {
        Number::Integer(id) => id.to_string(),
        _ => panic!("id must be a number in i64"),
    };
    let url = domain.to_owned() + "/api/v4/yaw/flows/" + &id_str;
    let headers = vec![("authorization".to_string(), authorization_token.to_string())];
    let request_data = NetworkingRequest::get(url, headers);
    let response = request(&request_data);
    let outputs = match response {
        Ok(response) => response.body.unwrap(),
        Err(error) => panic!("bad response {}", error.message),
    };
    Outputs::build(vec![ValuePresenter::Literal(
        LiteralValuePresenter::SingleLineField(SingleLineFieldValue::Value(outputs)),
    )])
}

fn extract_string_url(value_presenter: &ValuePresenter) -> &str {
    match value_presenter {
        ValuePresenter::Literal(LiteralValuePresenter::SingleLineField(
            SingleLineFieldValue::Value(str),
        )) => str,
        ValuePresenter::Literal(LiteralValuePresenter::SingleLineField(
            SingleLineFieldValue::Nil,
        )) => panic!("The url passed in is empty"),
        _ => unreachable!(
            "Unexpected a single line value presenter: got {:?}",
            value_presenter
        ),
    }
}

fn extract_number_id(value_presenter: &ValuePresenter) -> Number {
    match value_presenter {
        ValuePresenter::Literal(LiteralValuePresenter::NumericField(NumericFieldValue::Value(
            number,
        ))) => number.clone(),
        ValuePresenter::Literal(LiteralValuePresenter::NumericField(NumericFieldValue::Nil)) => {
            panic!("The process id passed in is empty")
        }
        _ => unreachable!(
            "Unexpected a single line value presenter: got {:?}",
            value_presenter
        ),
    }
}
fn extract_string_authorization(value_presenter: &ValuePresenter) -> &str {
    match value_presenter {
        ValuePresenter::Literal(LiteralValuePresenter::SingleLineField(
            SingleLineFieldValue::Value(str),
        )) => str,
        ValuePresenter::Literal(LiteralValuePresenter::SingleLineField(
            SingleLineFieldValue::Nil,
        )) => panic!("The incoming authorization certificate is empty"),
        _ => unreachable!(
            "Unexpected a single line value presenter: got {:?}",
            value_presenter
        ),
    }
}

program!(
    entrypoint,
    vec![
        FieldType::SingleLineField,
        FieldType::NumericField,
        FieldType::SingleLineField
    ]
);
