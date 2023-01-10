#![cfg_attr(not(test), no_main)]

pub mod parameter_analysis {
    use jet_programmable_rust_binding::{
        networking::{request, NetworkingRequest},
        outputs::Outputs,
        value_presenter::{
            literal_naive_value::{NumericFieldValue, SingleLineFieldValue},
            literal_value_presenter::LiteralValuePresenter,
            value::number::Number,
            ValuePresenter,
        },
    };

    pub fn extract_string_base_url(value_presenter: &ValuePresenter) -> &str {
        match value_presenter {
            ValuePresenter::Literal(LiteralValuePresenter::SingleLineField(
                SingleLineFieldValue::Value(str),
            )) => str,
            ValuePresenter::Literal(LiteralValuePresenter::SingleLineField(
                SingleLineFieldValue::Nil,
            )) => panic!("base url is a required parameter, please do not pass the empty value"),
            _ => unreachable!(
                "Unexpected a single line value presenter: got {:?}",
                value_presenter
            ),
        }
    }

    pub fn extract_number_flow_id(value_presenter: &ValuePresenter) -> String {
        match value_presenter {
            ValuePresenter::Literal(LiteralValuePresenter::NumericField(
                NumericFieldValue::Value(number),
            )) => {
                if let Number::Integer(id) = number {
                    id.to_string()
                } else {
                    panic!("The flow id must be of integer type");
                }
            }
            ValuePresenter::Literal(LiteralValuePresenter::NumericField(
                NumericFieldValue::Nil,
            )) => {
                panic!("flow id is a required parameter, please do not pass the empty value")
            }
            _ => unreachable!(
                "Unexpected a single line value presenter: got {:?}",
                value_presenter
            ),
        }
    }

    pub fn extract_number_node_id(value_presenter: &ValuePresenter) -> String {
        match value_presenter {
            ValuePresenter::Literal(LiteralValuePresenter::NumericField(
                NumericFieldValue::Value(number),
            )) => {
                if let Number::Integer(id) = number {
                    id.to_string()
                } else {
                    panic!("The node id must be of integer type");
                }
            }
            ValuePresenter::Literal(LiteralValuePresenter::NumericField(
                NumericFieldValue::Nil,
            )) => {
                panic!("node id is a required parameter, please do not pass the empty value")
            }
            _ => unreachable!(
                "Unexpected a single line value presenter: got {:?}",
                value_presenter
            ),
        }
    }

    pub fn extract_number_user_id(value_presenter: &ValuePresenter) -> String {
        match value_presenter {
            ValuePresenter::Literal(LiteralValuePresenter::NumericField(
                NumericFieldValue::Value(number),
            )) => {
                if let Number::Integer(id) = number {
                    id.to_string()
                } else {
                    panic!("The user id must be of integer type");
                }
            }
            ValuePresenter::Literal(LiteralValuePresenter::NumericField(
                NumericFieldValue::Nil,
            )) => {
                panic!("user id is a required parameter, please do not pass the empty value")
            }
            _ => unreachable!(
                "Unexpected a single line value presenter: got {:?}",
                value_presenter
            ),
        }
    }

    pub fn extract_number_journey_id(value_presenter: &ValuePresenter) -> String {
        match value_presenter {
            ValuePresenter::Literal(LiteralValuePresenter::NumericField(
                NumericFieldValue::Value(number),
            )) => {
                if let Number::Integer(id) = number {
                    id.to_string()
                } else {
                    panic!("The journey id must be of integer type");
                }
            }
            ValuePresenter::Literal(LiteralValuePresenter::NumericField(
                NumericFieldValue::Nil,
            )) => {
                panic!("journey id is a required parameter, please do not pass the empty value")
            }
            _ => unreachable!(
                "Unexpected a single line value presenter: got {:?}",
                value_presenter
            ),
        }
    }

    pub fn extract_string_authorization(value_presenter: &ValuePresenter) -> &str {
        match value_presenter {
            ValuePresenter::Literal(LiteralValuePresenter::SingleLineField(
                SingleLineFieldValue::Value(str),
            )) => str,
            ValuePresenter::Literal(LiteralValuePresenter::SingleLineField(
                SingleLineFieldValue::Nil,
            )) => panic!(
                "authorization token is a required parameter, please do not pass the empty value"
            ),
            _ => unreachable!(
                "Unexpected a single line value presenter: got {:?}",
                value_presenter
            ),
        }
    }

    pub fn request_to_outputs(request_data: NetworkingRequest) -> Outputs {
        let response = request(&request_data);
        let outputs = match response {
            Ok(response) => response.body.unwrap(),
            Err(error) => panic!("bad response {}", error.message),
        };
        Outputs::build(vec![ValuePresenter::Literal(
            LiteralValuePresenter::SingleLineField(SingleLineFieldValue::Value(outputs)),
        )])
    }
}
