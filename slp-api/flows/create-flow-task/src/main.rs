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

    let body = match extract_single_line_field_value(
        inputs
            .get(2)
            .expect("body is not passed with valid parameters"),
    ) {
        Ok(value) => value.unwrap_or(
            "The body passed in does not have a default value, please do not pass in nil or none",
        ),
        Err(err) => {
            panic!("Error: {}", err.message);
        }
    };

    let authorization_token = match extract_single_line_field_value(inputs.get(3).expect("authorization token is not passed with valid parameters")) {
        Ok(value) => {
            value.unwrap_or("The authorization token passed in does not have a default value, please do not pass in nil or none")
        },
        Err(err) => {
            panic!("Error: {}", err.message);
        }
    };
    let content_type = match extract_single_line_field_value(inputs.get(4).expect("content type is not passed with valid parameters")) {
        Ok(value) => {
            value.unwrap_or("The content type passed in does not have a default value, please do not pass in nil or none")
        },
        Err(err) => {
            panic!("Error: {}", err.message);
        }
    };

    let url = format!("{base_url}/api/v4/yaw/flows/{flow_id}/journeys");
    unsafe {
        jet_programmable_rust_binding::hostcalls::hostcall_logger_debug(url.as_ptr(), url.len());
        jet_programmable_rust_binding::hostcalls::hostcall_logger_debug(body.as_ptr(), body.len());
        jet_programmable_rust_binding::hostcalls::hostcall_logger_debug(
            authorization_token.as_ptr(),
            authorization_token.len(),
        );
    }
    let headers = vec![
        ("authorization".to_string(), authorization_token.to_string()),
        ("Content-Type".to_string(), content_type.to_string()),
    ];
    let request_data = NetworkingRequest::post(url, headers, Some(body.to_string()));
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
        FieldType::SingleLineField,
        FieldType::SingleLineField,
        FieldType::SingleLineField
    ]
);
