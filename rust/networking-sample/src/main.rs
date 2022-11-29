#![cfg_attr(not(test), no_main)]
use jet_programmable_rust_binding::hostcalls::*;
use serde::Serialize;
use std::slice;
use std::str;

#[derive(Serialize)]
struct RequestData {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ResponseData {
    code: u8,
    response: Option<Response>,
    message: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Response {
    status: u16,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

#[no_mangle]
pub fn run() {
    let json = RequestData {
        method: "GET".to_string(),
        url: "https://baidu.com/".to_string(),
        headers: Vec::new(),
        body: None,
    };

    let data = serde_json::to_string(&json).unwrap();
    let mut response_data = "";
    response_data = unsafe {
        let len = hostcall_request_inputs(data.as_ptr(), data.len(), response_data.as_ptr());
        let slice = slice::from_raw_parts(data_string.as_ptr(), len);
        str::from_utf8(slice).unwrap();
    };

    let response_data: RequestData = serde_json::from_str(response_data).unwrap();

    if response_data.code == 0 {
        println!("{:#?}", response_data.response);
    } else {
        println!("{:#?}", response_data.message);
    }
}
