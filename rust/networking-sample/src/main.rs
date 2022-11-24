#![cfg_attr(not(test), no_main)]
use jet_programmable_rust_binding::hostcalls::*;

#[no_mangle]
pub fn run(inputs: &str) {
    let json_str = String::from("{\"foo\":\"bar\"}");

    unsafe {
        hostcall_networking_request(json_str.as_ptr(), json_str.len());
        hostcall_set_outputs(inputs.as_ptr(), inputs.len());
    }
}
