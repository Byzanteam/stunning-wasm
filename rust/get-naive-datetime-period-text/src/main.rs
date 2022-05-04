#![cfg_attr(not(test), no_main)]

use core::str::FromStr;

use jet_programmable_rust_binding::{
    outputs::Outputs,
    program,
    value_presenter::{
        field_type::FieldType,
        field_value::{RadioButtonFieldValue, SingleLineFieldValue},
        literal::LiteralValuePresenter,
        value::options_value::OptionsValue,
    },
    value_presenter::{field_value::DateTimeFieldValue, ValuePresenter},
};

enum Frequency {
    Monthly,
    Bimonthly,
    Quarterly,
    Semiannually,
    Annually,
}

struct ParseFrequencyError;

impl FromStr for Frequency {
    type Err = ParseFrequencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "monthly" => Ok(Frequency::Monthly),
            "bimonthly" => Ok(Frequency::Bimonthly),
            "quarterly" => Ok(Frequency::Quarterly),
            "semiannually" => Ok(Frequency::Semiannually),
            "annually" => Ok(Frequency::Annually),
            _ => Err(ParseFrequencyError),
        }
    }
}

impl Frequency {
    pub fn to_string(&self, month: u8) -> String {
        match self {
            Frequency::Monthly => format!(" {} 月份", month),
            Frequency::Bimonthly => Self::to_bimonthly(month),
            Frequency::Quarterly => Self::to_quarterly(month),
            Frequency::Semiannually => Self::to_semiannually(month),
            Frequency::Annually => "全年".to_string(),
        }
    }

    fn to_bimonthly(month: u8) -> String {
        match month {
            1 | 2 => format!(" {} - {} 月份", 1, 2),
            3 | 4 => format!(" {} - {} 月份", 3, 4),
            5 | 6 => format!(" {} - {} 月份", 5, 6),
            7 | 8 => format!(" {} - {} 月份", 7, 8),
            9 | 10 => format!(" {} - {} 月份", 9, 10),
            11 | 12 => format!(" {} - {} 月份", 11, 12),
            _ => unreachable!(),
        }
    }

    fn to_quarterly(month: u8) -> String {
        match month {
            1 | 2 | 3 => "一季度".to_string(),
            4 | 5 | 6 => "二季度".to_string(),
            7 | 8 | 9 => "三季度".to_string(),
            10 | 11 | 12 => "四季度".to_string(),
            _ => unreachable!(),
        }
    }

    fn to_semiannually(month: u8) -> String {
        match month {
            1 | 2 | 3 | 4 | 5 | 6 => "上半年".to_string(),
            7 | 8 | 9 | 10 | 11 | 12 => "下半年".to_string(),
            _ => unreachable!(),
        }
    }
}

fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {
    let naive_datetime_vp = inputs.get(0).unwrap();
    let frequency_vp = inputs.get(1).unwrap();

    let (year, month) = extract_year_and_month(naive_datetime_vp);
    let frequency = extract_frequency(frequency_vp);

    let str = compose(year, month, frequency);

    Outputs::build(vec![ValuePresenter::Literal(
        LiteralValuePresenter::SingleLineField(SingleLineFieldValue::Value(str)),
    )])
}

fn extract_year_and_month(value_presenter: &ValuePresenter) -> (i32, u8) {
    match value_presenter {
        ValuePresenter::Literal(LiteralValuePresenter::DateTimeField(
            DateTimeFieldValue::Value(naive_date_time),
        )) => (naive_date_time.year, naive_date_time.month),
        _ => unreachable!(),
    }
}

fn extract_frequency(value_presenter: &ValuePresenter) -> Frequency {
    match value_presenter {
        ValuePresenter::Literal(LiteralValuePresenter::RadioButtonField(
            RadioButtonFieldValue::Value(OptionsValue {
                options,
                other: None,
            }),
        )) => match options.get(0) {
            Some(str) => match Frequency::from_str(str) {
                Ok(frequency) => frequency,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },

        _ => unreachable!(),
    }
}

fn compose(year: i32, month: u8, frequency: Frequency) -> String {
    format!("{} 年{}", year, frequency.to_string(month))
}

program!(
    entrypoint,
    vec![FieldType::DateTimeField, FieldType::RadioButtonField]
);
