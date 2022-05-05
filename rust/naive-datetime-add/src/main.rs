#![cfg_attr(not(test), no_main)]

use std::{
    ops::{Div, Rem},
    str::FromStr,
};

use jet_programmable_rust_binding::{
    outputs::Outputs,
    program,
    value_presenter::{
        field_type::FieldType,
        field_value::{DateTimeFieldValue, NumericFieldValue, RadioButtonFieldValue},
        literal::LiteralValuePresenter,
        value::{number::Number, options_value::OptionsValue},
    },
    value_presenter::{value::naive_date_time::NaiveDateTime, ValuePresenter},
};
use time::ext::NumericalDuration;
use time::{Date, Duration, Month, PrimitiveDateTime, Time};

#[derive(Debug, Clone)]
enum Unit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

#[derive(Debug)]
struct ParseUnitError;

impl FromStr for Unit {
    type Err = ParseUnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "second" | "seconds" => Ok(Unit::Seconds),
            "minute" | "minutes" => Ok(Unit::Minutes),
            "hour" | "hours" => Ok(Unit::Hours),
            "day" | "days" => Ok(Unit::Days),
            "month" | "months" => Ok(Unit::Months),
            "year" | "years" => Ok(Unit::Years),
            _ => Err(ParseUnitError),
        }
    }
}

fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {
    let ndt = match inputs.get(0) {
        Some(ValuePresenter::Literal(LiteralValuePresenter::DateTimeField(
            DateTimeFieldValue::Value(naive_datetime),
        ))) => naive_datetime,
        _ => unreachable!(),
    };

    let amount_to_add = match inputs.get(1) {
        Some(ValuePresenter::Literal(LiteralValuePresenter::NumericField(
            NumericFieldValue::Value(number),
        ))) => number,
        _ => unreachable!(),
    };

    let unit = match inputs.get(2) {
        Some(ValuePresenter::Literal(LiteralValuePresenter::RadioButtonField(
            RadioButtonFieldValue::Value(OptionsValue {
                options,
                other: None,
            }),
        ))) => options.get(0).unwrap(),
        _ => unreachable!(),
    };

    let pdt = to_primitive_datetime(ndt);
    let unit = Unit::from_str(unit).unwrap();

    let result = add(&pdt, amount_to_add, &unit);
    let result = to_naive_datetime(&result);

    Outputs::build(vec![ValuePresenter::Literal(
        LiteralValuePresenter::DateTimeField(DateTimeFieldValue::Value(result)),
    )])
}

fn add(
    primitive_datetime: &PrimitiveDateTime,
    amount_to_add: &Number,
    unit: &Unit,
) -> PrimitiveDateTime {
    match unit {
        Unit::Seconds => {
            let duration = match amount_to_add {
                Number::Integer(i) => i.seconds(),
                Number::Float(f) => f.seconds(),
            };
            add_duration(primitive_datetime, duration)
        }
        Unit::Minutes => {
            let duration = match amount_to_add {
                Number::Integer(i) => i.minutes(),
                Number::Float(f) => f.minutes(),
            };
            add_duration(primitive_datetime, duration)
        }
        Unit::Hours => {
            let duration = match amount_to_add {
                Number::Integer(i) => i.hours(),
                Number::Float(f) => f.hours(),
            };
            add_duration(primitive_datetime, duration)
        }
        Unit::Days => {
            let duration = match amount_to_add {
                Number::Integer(i) => i.days(),
                Number::Float(f) => f.days(),
            };
            add_duration(primitive_datetime, duration)
        }
        Unit::Months => match amount_to_add {
            Number::Integer(i) => add_months(primitive_datetime, *i as i32),
            Number::Float(f) => add_months(primitive_datetime, *f as i32),
        },
        Unit::Years => match amount_to_add {
            Number::Integer(i) => add_years(primitive_datetime, *i as i32),
            Number::Float(f) => add_years(primitive_datetime, *f as i32),
        },
    }
}

fn to_primitive_datetime(naive_datetime: &NaiveDateTime) -> PrimitiveDateTime {
    let date = Date::from_calendar_date(
        naive_datetime.year,
        Month::try_from(naive_datetime.month).unwrap(),
        naive_datetime.day,
    )
    .unwrap();
    let time = Time::from_hms_nano(
        naive_datetime.hour,
        naive_datetime.minute,
        naive_datetime.second,
        naive_datetime.nanosecond,
    )
    .unwrap();

    PrimitiveDateTime::new(date, time)
}

fn to_naive_datetime(primitive_datetime: &PrimitiveDateTime) -> NaiveDateTime {
    NaiveDateTime {
        year: primitive_datetime.year(),
        month: primitive_datetime.month() as u8,
        day: primitive_datetime.day(),
        hour: primitive_datetime.hour(),
        minute: primitive_datetime.minute(),
        second: primitive_datetime.second(),
        nanosecond: primitive_datetime.nanosecond(),
    }
}

fn add_duration(primitive_datetime: &PrimitiveDateTime, duration: Duration) -> PrimitiveDateTime {
    primitive_datetime.saturating_add(duration)
}

fn add_months(primitive_datetime: &PrimitiveDateTime, months: i32) -> PrimitiveDateTime {
    let month_amount = primitive_datetime.month() as i32 + months;

    let years_to_add = month_amount.div(12);
    let month = Month::try_from(month_amount.rem(12) as u8).unwrap();

    let pdt = primitive_datetime
        .replace_day(1)
        .and_then(|pdt| pdt.replace_year(pdt.year() + years_to_add))
        .and_then(|pdt| pdt.replace_month(month))
        .unwrap();

    set_to_closet_day(&pdt, primitive_datetime.day())
}

fn add_years(primitive_datetime: &PrimitiveDateTime, years: i32) -> PrimitiveDateTime {
    let pdt = primitive_datetime
        .replace_day(1)
        .and_then(|pdt| pdt.replace_year(pdt.year() + years))
        .unwrap();

    set_to_closet_day(&pdt, primitive_datetime.day())
}

fn set_to_closet_day(primitive_datetime: &PrimitiveDateTime, day: u8) -> PrimitiveDateTime {
    let mut day = day;

    loop {
        match primitive_datetime.replace_day(day) {
            Ok(pdt) => {
                break pdt;
            }
            Err(_) => {
                day -= 1;
            }
        }
    }
}

program!(
    entrypoint,
    vec![
        FieldType::DateTimeField,
        FieldType::NumericField,
        FieldType::RadioButtonField,
    ]
);

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn test_entrypoint() {
        let naive_datetime = date_time_field_value(NaiveDateTime {
            year: 2020,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            nanosecond: 0,
        });
        let amount_to_add = numeric_field_value(Number::Integer(1));
        let unit = radio_button_field_value("seconds".to_string());

        let outputs = entrypoint(vec![naive_datetime, amount_to_add, unit]);

        assert!(matches!(
            outputs.0.as_slice(),
            [ValuePresenter::Literal(
                LiteralValuePresenter::DateTimeField(DateTimeFieldValue::Value(NaiveDateTime {
                    year: 2020,
                    month: 1,
                    day: 1,
                    hour: 0,
                    minute: 0,
                    second: 1,
                    nanosecond: 0,
                }))
            )]
        ));
    }

    fn date_time_field_value(naive_datetime: NaiveDateTime) -> ValuePresenter {
        ValuePresenter::Literal(LiteralValuePresenter::DateTimeField(
            DateTimeFieldValue::Value(naive_datetime),
        ))
    }

    fn numeric_field_value(number: Number) -> ValuePresenter {
        ValuePresenter::Literal(LiteralValuePresenter::NumericField(
            NumericFieldValue::Value(number),
        ))
    }

    fn radio_button_field_value(option: String) -> ValuePresenter {
        ValuePresenter::Literal(LiteralValuePresenter::RadioButtonField(
            RadioButtonFieldValue::Value(OptionsValue {
                options: vec![option],
                other: None,
            }),
        ))
    }

    #[test]
    fn test_add_seconds() {
        {
            let primitive_datetime = datetime!(2020-01-01 00:00:00);
            let amount_to_add = Number::Integer(1);
            let unit = Unit::Seconds;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2020-01-01 00:00:01));
        }

        // float amount
        {
            let primitive_datetime = datetime!(2020-01-01 00:00:00);
            let amount_to_add = Number::Float(1.2);
            let unit = Unit::Seconds;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2020-01-01 00:00:01.2));
        }

        // cross a day
        {
            let primitive_datetime = datetime!(2020-01-01 23:59:59);
            let amount_to_add = Number::Integer(1);
            let unit = Unit::Seconds;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2020-01-02 00:00:00));
        }

        // cross a month
        {
            let primitive_datetime = datetime!(2020-02-29 23:59:59);
            let amount_to_add = Number::Integer(1);
            let unit = Unit::Seconds;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2020-03-01 00:00:00));
        }
    }

    #[test]
    fn test_add_minutes() {
        {
            let primitive_datetime = datetime!(2020-01-01 00:00:00);
            let amount_to_add = Number::Integer(1);
            let unit = Unit::Minutes;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2020-01-01 00:01:00));
        }

        // float amount
        {
            let primitive_datetime = datetime!(2020-01-01 00:00:00);
            let amount_to_add = Number::Float(1.52);
            let unit = Unit::Minutes;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2020-01-01 00:01:31.2));
        }
    }

    #[test]
    fn test_add_hours() {
        {
            let primitive_datetime = datetime!(2020-01-01 00:00:00);
            let amount_to_add = Number::Integer(1);
            let unit = Unit::Hours;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2020-01-01 01:00:00));
        }

        // float amount
        {
            let primitive_datetime = datetime!(2020-01-01 00:00:00);
            let amount_to_add = Number::Float(1.52);
            let unit = Unit::Hours;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2020-01-01 01:31:12));
        }
    }

    #[test]
    fn test_add_days() {
        {
            let primitive_datetime = datetime!(2020-01-01 00:00:00);
            let amount_to_add = Number::Integer(1);
            let unit = Unit::Days;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2020-01-02 00:00:00));
        }

        // float amount
        {
            let primitive_datetime = datetime!(2020-01-01 00:00:00);
            let amount_to_add = Number::Float(1.52);
            let unit = Unit::Days;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2020-01-02 12:28:48));
        }

        // cross a month
        {
            let primitive_datetime = datetime!(2020-01-31 00:00:00);
            let amount_to_add = Number::Integer(1);
            let unit = Unit::Days;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2020-02-01 00:00:00));
        }
    }

    #[test]
    fn test_add_months() {
        {
            let primitive_datetime = datetime!(2020-01-01 00:00:00);
            let amount_to_add = Number::Integer(1);
            let unit = Unit::Months;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2020-02-01 00:00:00));
        }

        {
            let primitive_datetime = datetime!(2020-01-31 00:00:00);
            let amount_to_add = Number::Integer(3);
            let unit = Unit::Months;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2020-04-30 00:00:00));
        }

        {
            let primitive_datetime = datetime!(2020-01-31 00:00:00);
            let amount_to_add = Number::Integer(13);
            let unit = Unit::Months;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2021-02-28 00:00:00));
        }

        // float amount
        {
            let primitive_datetime = datetime!(2020-01-01 00:00:00);
            let amount_to_add = Number::Float(1.99);
            let unit = Unit::Months;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2020-02-01 00:00:00));
        }

        // leap year
        {
            let primitive_datetime = datetime!(2021-01-31 00:00:00);
            let amount_to_add = Number::Integer(1);
            let unit = Unit::Months;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2021-02-28 00:00:00));
        }
    }

    #[test]
    fn test_add_years() {
        {
            let primitive_datetime = datetime!(2020-01-01 00:00:00);
            let amount_to_add = Number::Integer(1);
            let unit = Unit::Years;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2021-01-01 00:00:00));
        }

        // leap year
        {
            let primitive_datetime = datetime!(2020-02-29 00:00:00);
            let amount_to_add = Number::Integer(3);
            let unit = Unit::Years;

            let pdt = add(&primitive_datetime, &amount_to_add, &unit);

            assert_eq!(pdt, datetime!(2023-02-28 00:00:00));
        }
    }
}
