#![no_main]
use serde::{Serialize};

// use jet_programmable_rust_binding::hostcalls::hostcall_set_outputs;
extern "C" {
    pub fn hostcall_set_outputs(outputs_ptr: *const u8, outputs_len: usize);
    pub fn hostcall_request_inputs(inputs_ptr: *const u8, inputs_len: usize);
    pub fn hostcall_response_outputs(outputs_ptr: *const u8, outputs_len: usize);
}

#[derive(Serialize)]
struct RequestData {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

// #[derive(Deserialize)]
// struct ResponseData {
//     status: u16,
//     headers: Vec<(String, String)>,
//     body: Option<String>,
// }
#[no_mangle]
pub fn run() {
    let json = RequestData {
        method: "GET".to_string(),
        url: "https://rust-lang.org/".to_string(),
        headers: Vec::new(),
        body: None,
    };
    let data = serde_json::to_string(&json).unwrap();
    unsafe {
        hostcall_request_inputs(data.as_ptr(), data.len());
    }
}

#[no_mangle]
pub fn response(data: &str) {
    unsafe{
        hostcall_response_outputs(data.as_ptr(), data.len());
    }
}