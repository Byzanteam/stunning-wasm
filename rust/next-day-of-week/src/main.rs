#![cfg_attr(not(test), no_main)]

use core::convert::TryFrom;
use time::{Date, Duration, Month, PrimitiveDateTime, Time};

use jet_programmable_rust_binding::{
    outputs::Outputs,
    program,
    value_presenter::{
        field_type::FieldType, literal_naive_value::DateTimeFieldValue,
        literal_value_presenter::LiteralValuePresenter,
    },
    value_presenter::{value::naive_date_time::NaiveDateTime, ValuePresenter},
};

const ONE_WEEK: Duration = Duration::WEEK;

fn entrypoint(inputs: Vec<ValuePresenter>) -> Outputs {
    let ndt = match inputs.get(0) {
        Some(ValuePresenter::Literal(LiteralValuePresenter::DateTimeField(
            DateTimeFieldValue::Value(naive_date_time),
        ))) => naive_date_time,
        _ => unreachable!(),
    };

    let pdt = to_primitive_date_time(ndt).saturating_add(ONE_WEEK);

    let vp = ValuePresenter::Literal(LiteralValuePresenter::DateTimeField(
        DateTimeFieldValue::Value(NaiveDateTime {
            year: pdt.year(),
            month: pdt.month() as u8,
            day: pdt.day(),
            hour: pdt.hour(),
            minute: pdt.minute(),
            second: pdt.second(),
            nanosecond: pdt.nanosecond(),
        }),
    ));

    Outputs::build(vec![vp])
}

fn to_primitive_date_time(naive_date_time: &NaiveDateTime) -> PrimitiveDateTime {
    let date = Date::from_calendar_date(
        naive_date_time.year,
        Month::try_from(naive_date_time.month).unwrap(),
        naive_date_time.day,
    )
    .unwrap();
    let time = Time::from_hms_nano(
        naive_date_time.hour,
        naive_date_time.minute,
        naive_date_time.second,
        naive_date_time.nanosecond,
    )
    .unwrap();

    PrimitiveDateTime::new(date, time)
}

program!(entrypoint, vec![FieldType::DateTimeField]);
