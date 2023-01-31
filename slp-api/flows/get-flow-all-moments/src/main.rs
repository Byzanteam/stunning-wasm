#![cfg_attr(not(test), no_main)]

use flow_util::flow_util::{
    extract_numeric_field_value, extract_single_line_field_value, number_to_string_id,
    response_to_outputs,
};
use jet_programmable_rust_binding::{
    networking::{request, NetworkingRequest},
    outputs::Outputs,
    program,
    value_presenter::{field_type::FieldType, ValuePresenter},
};

fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {
    let base_url = match extract_single_line_field_value(inputs.get(0).expect("base url is not passed with valid parameters")) {
        Ok(value) => {
            value.unwrap_or("The bese url passed in does not have a default value, please do not pass in nil or none")
        },
        Err(err) => {
            panic!("Error: {}", err.message);
        }
    };

    let flow_id = match extract_numeric_field_value(
        inputs
            .get(1)
            .expect("flow id is not passed with valid parameters"),
    ) {
        Ok(value) => {
            if let Some(number_value) = value {
                number_to_string_id(number_value)
            } else {
                panic!("The flow id passed in does not have a default value, please do not pass in nil or none")
            }
        }
        Err(err) => {
            panic!("Error: {}", err.message);
        }
    };

    let authorization_token = match extract_single_line_field_value(inputs.get(2).expect("base url is not passed with valid parameters")) {
        Ok(value) => {
            value.unwrap_or("The authorization token passed in does not have a default value, please do not pass in nil or none")
        },
        Err(err) => {
            panic!("Error: {}", err.message);
        }
    };
    let url = format!("{base_url}/api/v4/yaw/journeys/{flow_id}/moments");
    let headers = vec![("authorization".to_string(), authorization_token.to_string())];
    let request_data = NetworkingRequest::get(url, headers);
    let response_data = request(&request_data);
    match response_data {
        Ok(response) => response_to_outputs(response),
        Err(err) => panic!("error code: {}, error message: {}", err.code, err.message),
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
