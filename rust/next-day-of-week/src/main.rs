#![no_main]

use chrono::{Duration, NaiveDateTime};
use eetf::{Binary, List, Term};
use std::io::Cursor;
use std::str;

extern "C" {
    pub fn hostcall_set_outputs(outputs_ptr: *const u8, outputs_len: usize);
}

#[no_mangle]
pub fn run(params: &str) {
    let bytes = params.as_bytes();
    let term = Term::decode(Cursor::new(&bytes)).unwrap();

    match term {
        Term::List(ref list) => {
            let slice = &list.elements;

            match slice.first() {
                Some(Term::Binary(date_time_value)) => next_day_of_week(date_time_value),
                _ => (),
            }
        }
        _ => (),
    }
}

const DATE_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

fn next_day_of_week(date_time_value: &Binary) {
    let ndt_str = str::from_utf8(&date_time_value.bytes).unwrap();
    let ndt = NaiveDateTime::parse_from_str(ndt_str, DATE_TIME_FORMAT).unwrap();

    let next_day = ndt + Duration::days(7);
    let next_day = next_day.format(DATE_TIME_FORMAT).to_string();
    let next_day = Binary::from(next_day.into_bytes());
    let next_day = Term::from(next_day);

    let outputs = List::from(vec![next_day]);
    let term = Term::from(outputs);

    let mut buf = Vec::new();

    term.encode(&mut buf).unwrap();

    unsafe {
        hostcall_set_outputs(buf.as_ptr(), buf.len());
    }
}
