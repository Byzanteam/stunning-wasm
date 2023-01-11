pub mod flow_util {
    use jet_programmable_rust_binding::{
        networking::NetworkingResponse,
        outputs::Outputs,
        value_presenter::{
            literal_naive_value::{NumericFieldValue, SingleLineFieldValue},
            literal_value_presenter::LiteralValuePresenter,
            value::number::Number,
            ValuePresenter,
        },
    };

    #[derive(Debug)]
    pub struct ExtractError {
        pub message: String,
    }

    impl ExtractError {
        pub fn new(message: String) -> Self {
            Self { message }
        }
    }
    pub fn extract_single_line_field_value(
        value_presenter: &ValuePresenter,
    ) -> Result<Option<&str>, ExtractError> {
        match value_presenter {
            ValuePresenter::Literal(LiteralValuePresenter::SingleLineField(
                SingleLineFieldValue::Value(str),
            )) => Ok(Some(str)),
            ValuePresenter::Literal(LiteralValuePresenter::SingleLineField(
                SingleLineFieldValue::Nil,
            )) => Ok(None),
            _ => Err(ExtractError::new(
                "Unexpected a single line value presenter".to_string(),
            )),
        }
    }

    //The reason for not returning an integer type is to prevent us
    //from using a floating point number or other numeric type later
    pub fn extract_numeric_field_value(
        value_presenter: &ValuePresenter,
    ) -> Result<Option<&Number>, ExtractError> {
        match value_presenter {
            ValuePresenter::Literal(LiteralValuePresenter::NumericField(
                NumericFieldValue::Value(number),
            )) => Ok(Some(number)),
            ValuePresenter::Literal(LiteralValuePresenter::NumericField(
                NumericFieldValue::Nil,
            )) => Ok(None),
            _ => Err(ExtractError::new(
                "Unexpected a numberic value presenter".to_string(),
            )),
        }
    }

    //According to the practical experience,
    //it is best to implement a method to convert the enum of Number to string,
    //but we will implement this function directly here
    pub fn number_to_string_id(number_value: &Number) -> String {
        match number_value {
            Number::Integer(num) => num.to_string(),
            _ => panic!("This is the id related to the network request, the parameter type must be integer type") }
    }

    pub fn response_to_outputs(response: NetworkingResponse) -> Outputs {
        Outputs::build(vec![ValuePresenter::Literal(
            LiteralValuePresenter::SingleLineField(SingleLineFieldValue::Value(
                response.body.unwrap(),
            )),
        )])
    }
}
