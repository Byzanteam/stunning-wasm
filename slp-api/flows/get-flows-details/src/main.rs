#![cfg_attr(not(test), no_main)]

use jet_programmable_rust_binding::{
    networking::NetworkingRequest,
    outputs::Outputs,
    program,
    value_presenter::{field_type::FieldType, ValuePresenter},
};
use slp_api_public::parameter_analysis::{
    extract_number_flow_id, extract_string_authorization, extract_string_base_url,
    request_to_outputs,
};

fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {
    let base_url = extract_string_base_url(
        inputs
            .get(0)
            .expect("base url is not passed with valid parameters"),
    );
    let flow_id = extract_number_flow_id(
        inputs
            .get(1)
            .expect("flow id is not passed with valid parameters"),
    );
    let authorization_token = extract_string_authorization(
        inputs
            .get(2)
            .expect("authorization token is not passed with valid parameters"),
    );
    let url = format!("{}/api/v4/yaw/flows/{}", base_url, flow_id);
    let headers = vec![("authorization".to_string(), authorization_token.to_string())];
    let request_data = NetworkingRequest::get(url, headers);
    request_to_outputs(request_data)
}

program!(
    entrypoint,
    vec![
        FieldType::SingleLineField,
        FieldType::NumericField,
        FieldType::SingleLineField
    ]
);
