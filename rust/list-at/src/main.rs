#![cfg_attr(not(test), no_main)]

use jet_programmable_rust_binding::hostcalls::hostcall_set_outputs;

#[no_mangle]
pub fn run(inputs: &str) {
    unsafe {
        hostcall_set_outputs(inputs.as_ptr(), inputs.len());
    }
}
