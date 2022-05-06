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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_monthly() {
        assert_eq!(compose(2021, 1, Frequency::Monthly), "2021 年 1 月份");
        assert_eq!(compose(2021, 2, Frequency::Monthly), "2021 年 2 月份");
    }

    #[test]
    fn test_compose_bimonthly() {
        assert_eq!(compose(2021, 1, Frequency::Bimonthly), "2021 年 1 - 2 月份");
        assert_eq!(compose(2021, 2, Frequency::Bimonthly), "2021 年 1 - 2 月份");
        assert_eq!(compose(2021, 3, Frequency::Bimonthly), "2021 年 3 - 4 月份");
        assert_eq!(
            compose(2021, 12, Frequency::Bimonthly),
            "2021 年 11 - 12 月份"
        );
    }

    #[test]
    fn test_compose_quarterly() {
        assert_eq!(compose(2021, 1, Frequency::Quarterly), "2021 年一季度");
        assert_eq!(compose(2021, 2, Frequency::Quarterly), "2021 年一季度");
        assert_eq!(compose(2021, 3, Frequency::Quarterly), "2021 年一季度");
        assert_eq!(compose(2021, 4, Frequency::Quarterly), "2021 年二季度");
        assert_eq!(compose(2021, 5, Frequency::Quarterly), "2021 年二季度");
        assert_eq!(compose(2021, 6, Frequency::Quarterly), "2021 年二季度");
        assert_eq!(compose(2021, 12, Frequency::Quarterly), "2021 年四季度");
    }

    #[test]
    fn test_compose_semiannually() {
        assert_eq!(compose(2021, 1, Frequency::Semiannually), "2021 年上半年");
        assert_eq!(compose(2021, 6, Frequency::Semiannually), "2021 年上半年");
        assert_eq!(compose(2021, 12, Frequency::Semiannually), "2021 年下半年");
    }

    #[test]
    fn test_compose_annually() {
        assert_eq!(compose(2021, 1, Frequency::Annually), "2021 年全年");
        assert_eq!(compose(2021, 6, Frequency::Annually), "2021 年全年");
        assert_eq!(compose(2021, 12, Frequency::Annually), "2021 年全年");
    }
}
