pub mod flow_util {

    use jet_programmable_rust_binding::{
        networking::{NetworkingHeaders, NetworkingResponse},
        outputs::Outputs,
        value_presenter::{
            literal_list_value::SingleLineListFieldValue,
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

    pub fn extract_single_line_list_field_value(
        value_presenter: &ValuePresenter,
    ) -> Result<Option<Vec<String>>, ExtractError> {
        match value_presenter {
            ValuePresenter::Literal(LiteralValuePresenter::SingleLineListField(
                SingleLineListFieldValue::Value(values),
            )) => {
                let strings: Vec<String> = values
                    .iter()
                    .filter_map(|value| match value {
                        SingleLineFieldValue::Value(s) => Some(s.to_owned()),
                        SingleLineFieldValue::Nil => None,
                    })
                    .collect();
                Ok(Some(strings))
            }
            ValuePresenter::Literal(LiteralValuePresenter::SingleLineListField(
                SingleLineListFieldValue::Nil,
            )) => Ok(None),
            _ => Err(ExtractError::new(
                "Unexpected a string list value parameter".to_string(),
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
    pub fn pop_params(
        inputs: Vec<ValuePresenter>,
    ) -> Result<(String, NetworkingHeaders, Vec<ValuePresenter>), ExtractError> {
        let base_url = extract_single_line_field_value(inputs.get(0).unwrap())?
            .ok_or_else(|| ExtractError::new(String::from("base url cannot be none or nil")))?
            .to_string();
        let headers = extract_single_line_list_field_value(inputs.get(1).unwrap())?
            .ok_or_else(|| ExtractError::new(String::from("headers cannot be none or nil")))?;
        let header_pairs = headers
            .chunks(2)
            .map(|chunk| (chunk[0].to_owned(), chunk[1].to_owned()))
            .collect();
        let remaining = inputs.into_iter().skip(2).collect();
        Ok((base_url, header_pairs, remaining))
    }

    pub fn extract_value(
        value_presenter: &ValuePresenter,
        field_name: &str,
    ) -> Result<String, ExtractError> {
        match value_presenter {
            ValuePresenter::Literal(LiteralValuePresenter::NumericField(_value)) => {
                extract_numeric_field_value(value_presenter)
                    .and_then(|value| {
                        value.ok_or_else(|| ExtractError::new(format!("The {field_name} of type NumericField cannot be nil or none")))
                            .map(number_to_string_id)
                    })
            },
            ValuePresenter::Literal(LiteralValuePresenter::SingleLineField(_value)) => {
                extract_single_line_field_value(value_presenter)
                    .and_then(|value| {
                        value.ok_or_else(|| ExtractError::new(format!("The {field_name} of type SingleLineField cannot be nil or none")))
                            .map(|x| x.to_string())
                    })
            },
            _ => Err(ExtractError::new(format!("The value_presenter for `${field_name}` must be one of NumericField and SingleLineField, but got {value_presenter:?}")))
        }
    }
}
