#![cfg_attr(not(test), no_main)]

use core::str::FromStr;
use std::{convert::TryFrom, ops::Rem};

use jet_programmable_rust_binding::{
    outputs::Outputs,
    program,
    value_presenter::{
        field_type::FieldType,
        literal_naive_value::{DateTimeFieldValue, RadioButtonFieldValue},
        literal_value_presenter::LiteralValuePresenter,
        value::{naive_date_time::NaiveDateTime, options_value::OptionsValue},
        ValuePresenter,
    },
};
use time::{macros::time, util::days_in_year_month, Date, Month, PrimitiveDateTime, Time};

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

fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {
    let naive_datetime_vp = inputs.get(0).unwrap();
    let frequency_vp = inputs.get(1).unwrap();

    let (year, month) = extract_year_and_month(naive_datetime_vp);
    let frequency = extract_frequency(frequency_vp);

    let (start, end) = compute_range(year, month, frequency);

    Outputs::build(vec![
        ValuePresenter::Literal(LiteralValuePresenter::DateTimeField(
            DateTimeFieldValue::Value(start),
        )),
        ValuePresenter::Literal(LiteralValuePresenter::DateTimeField(
            DateTimeFieldValue::Value(end),
        )),
    ])
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

const START_TIME: Time = time!(00:00:00);
const END_TIME: Time = time!(23:59:59);

fn compute_range(year: i32, month: u8, frequency: Frequency) -> (NaiveDateTime, NaiveDateTime) {
    let (start_date, end_date) = compute_date_range(year, month, frequency);

    let start = PrimitiveDateTime::new(start_date, START_TIME);
    let end = PrimitiveDateTime::new(end_date, END_TIME);

    (
        NaiveDateTime {
            year: start.year(),
            month: start.month() as u8,
            day: start.day(),
            hour: start.hour(),
            minute: start.minute(),
            second: start.second(),
            nanosecond: start.nanosecond(),
        },
        NaiveDateTime {
            year: end.year(),
            month: end.month() as u8,
            day: end.day(),
            hour: end.hour(),
            minute: end.minute(),
            second: end.second(),
            nanosecond: end.nanosecond(),
        },
    )
}

fn compute_date_range(year: i32, month: u8, frequency: Frequency) -> (Date, Date) {
    match frequency {
        Frequency::Monthly => {
            let month = Month::try_from(month).unwrap();

            (
                get_first_day_of_month(year, month),
                get_last_day_of_month(year, month),
            )
        }
        Frequency::Bimonthly => {
            let (start_month, end_month) = match month.rem(2) {
                0 => {
                    let end_month = Month::try_from(month).unwrap();

                    (end_month.previous(), end_month)
                }
                1 => {
                    let start_month = Month::try_from(month).unwrap();
                    (start_month, start_month.next())
                }
                _ => unreachable!(),
            };

            (
                get_first_day_of_month(year, start_month),
                get_last_day_of_month(year, end_month),
            )
        }
        Frequency::Quarterly => {
            let (start_month, end_month) = match month.rem(3) {
                0 => {
                    let end_month = Month::try_from(month).unwrap();

                    (end_month.previous().previous(), end_month)
                }
                1 => {
                    let start_month = Month::try_from(month).unwrap();
                    (start_month, start_month.next().next())
                }
                2 => {
                    let month = Month::try_from(month).unwrap();
                    (month.previous(), month.next())
                }
                _ => unreachable!(),
            };

            (
                get_first_day_of_month(year, start_month),
                get_last_day_of_month(year, end_month),
            )
        }
        Frequency::Semiannually => {
            let (start_month, end_month) = match month {
                1..=6 => (Month::January, Month::June),
                7..=12 => (Month::July, Month::December),
                _ => unreachable!(),
            };

            (
                get_first_day_of_month(year, start_month),
                get_last_day_of_month(year, end_month),
            )
        }
        Frequency::Annually => (
            get_first_day_of_month(year, Month::January),
            get_last_day_of_month(year, Month::December),
        ),
    }
}

fn get_first_day_of_month(year: i32, month: Month) -> Date {
    Date::from_calendar_date(year, month, 1).unwrap()
}

fn get_last_day_of_month(year: i32, month: Month) -> Date {
    let last_day = days_in_year_month(year, month);

    Date::from_calendar_date(year, month, last_day).unwrap()
}

program!(
    entrypoint,
    vec![FieldType::DateTimeField, FieldType::RadioButtonField]
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monthly_frequency() {
        assert_range(
            2020,
            1,
            Frequency::Monthly,
            (
                new_naive_datetime(2020, 1, 1, 0, 0, 0),
                new_naive_datetime(2020, 1, 31, 23, 59, 59),
            ),
        );

        // leap year
        assert_range(
            2020,
            2,
            Frequency::Monthly,
            (
                new_naive_datetime(2020, 2, 1, 0, 0, 0),
                new_naive_datetime(2020, 2, 29, 23, 59, 59),
            ),
        );

        assert_range(
            2021,
            2,
            Frequency::Monthly,
            (
                new_naive_datetime(2021, 2, 1, 0, 0, 0),
                new_naive_datetime(2021, 2, 28, 23, 59, 59),
            ),
        );
    }

    #[test]
    fn test_bimonthly_frequency() {
        assert_range(
            2020,
            1,
            Frequency::Bimonthly,
            (
                new_naive_datetime(2020, 1, 1, 0, 0, 0),
                new_naive_datetime(2020, 2, 29, 23, 59, 59),
            ),
        );

        assert_range(
            2020,
            2,
            Frequency::Bimonthly,
            (
                new_naive_datetime(2020, 1, 1, 0, 0, 0),
                new_naive_datetime(2020, 2, 29, 23, 59, 59),
            ),
        );

        assert_range(
            2020,
            12,
            Frequency::Bimonthly,
            (
                new_naive_datetime(2020, 11, 1, 0, 0, 0),
                new_naive_datetime(2020, 12, 31, 23, 59, 59),
            ),
        );
    }

    #[test]
    fn test_quarterly_frequency() {
        assert_range(
            2020,
            1,
            Frequency::Quarterly,
            (
                new_naive_datetime(2020, 1, 1, 0, 0, 0),
                new_naive_datetime(2020, 3, 31, 23, 59, 59),
            ),
        );

        assert_range(
            2020,
            2,
            Frequency::Quarterly,
            (
                new_naive_datetime(2020, 1, 1, 0, 0, 0),
                new_naive_datetime(2020, 3, 31, 23, 59, 59),
            ),
        );

        assert_range(
            2020,
            3,
            Frequency::Quarterly,
            (
                new_naive_datetime(2020, 1, 1, 0, 0, 0),
                new_naive_datetime(2020, 3, 31, 23, 59, 59),
            ),
        );

        assert_range(
            2020,
            8,
            Frequency::Quarterly,
            (
                new_naive_datetime(2020, 7, 1, 0, 0, 0),
                new_naive_datetime(2020, 9, 30, 23, 59, 59),
            ),
        );

        assert_range(
            2020,
            12,
            Frequency::Quarterly,
            (
                new_naive_datetime(2020, 10, 1, 0, 0, 0),
                new_naive_datetime(2020, 12, 31, 23, 59, 59),
            ),
        );
    }

    #[test]
    fn test_semiannually_frequency() {
        assert_range(
            2020,
            1,
            Frequency::Semiannually,
            (
                new_naive_datetime(2020, 1, 1, 0, 0, 0),
                new_naive_datetime(2020, 6, 30, 23, 59, 59),
            ),
        );

        assert_range(
            2020,
            2,
            Frequency::Semiannually,
            (
                new_naive_datetime(2020, 1, 1, 0, 0, 0),
                new_naive_datetime(2020, 6, 30, 23, 59, 59),
            ),
        );

        assert_range(
            2020,
            8,
            Frequency::Semiannually,
            (
                new_naive_datetime(2020, 7, 1, 0, 0, 0),
                new_naive_datetime(2020, 12, 31, 23, 59, 59),
            ),
        );

        assert_range(
            2020,
            12,
            Frequency::Semiannually,
            (
                new_naive_datetime(2020, 7, 1, 0, 0, 0),
                new_naive_datetime(2020, 12, 31, 23, 59, 59),
            ),
        );
    }

    #[test]
    fn test_annually_frequency() {
        assert_range(
            2020,
            1,
            Frequency::Annually,
            (
                new_naive_datetime(2020, 1, 1, 0, 0, 0),
                new_naive_datetime(2020, 12, 31, 23, 59, 59),
            ),
        );

        assert_range(
            2020,
            12,
            Frequency::Annually,
            (
                new_naive_datetime(2020, 1, 1, 0, 0, 0),
                new_naive_datetime(2020, 12, 31, 23, 59, 59),
            ),
        );
    }

    fn new_naive_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> NaiveDateTime {
        NaiveDateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanosecond: 0,
        }
    }

    fn assert_range(
        year: i32,
        month: u8,
        frequency: Frequency,
        (expected_start, expected_end): (NaiveDateTime, NaiveDateTime),
    ) {
        let (start, end) = compute_range(year, month, frequency);

        assert_eq!(start, expected_start);
        assert_eq!(end, expected_end);
    }
}
