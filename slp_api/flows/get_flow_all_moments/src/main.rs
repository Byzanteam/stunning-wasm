#![cfg_attr(not(test), no_main)]

use flow_util::flow_util::{extract_value, pop_params, response_to_outputs};
use jet_programmable_rust_binding::{
    networking::{request, NetworkingRequest},
    outputs::Outputs,
    program,
    value_presenter::{field_type::FieldType, ValuePresenter},
};

fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {
    let (base_url, headers, other_value) = pop_params(inputs).unwrap();
    let flow_id = extract_value(other_value.get(0).unwrap(), "flow_id")
        .unwrap_or_else(|err| panic!("{}", err.message));
    let url = format!("{base_url}/api/v4/yaw/journeys/{flow_id}/moments");
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
        FieldType::SingleLineListField,
        FieldType::NumericField,
    ]
);
