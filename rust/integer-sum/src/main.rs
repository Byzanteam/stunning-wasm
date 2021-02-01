#![no_main]

use eetf::{FixInteger, Term, List};
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

            if slice.len() == 2 {
                let arg1 = match slice.first() {
                    Some(Term::FixInteger(fix_integer)) => fix_integer.value,
                    _ => 0,
                };
                let arg2 = match slice.last() {
                    Some(Term::FixInteger(fix_integer)) => fix_integer.value,
                    _ => 0,
                };

                let sum = arg1 + arg2;
                let sum = FixInteger::from(sum);
                let sum = Term::from(sum);

                let outputs = List::from(vec![sum]);
                let term = Term::from(outputs);

                let mut buf = Vec::new();

                term.encode(&mut buf).unwrap();

                unsafe {
                    hostcall_set_outputs(buf.as_ptr(), buf.len());
                }
            }
        }
        _ => (),
    }
}
