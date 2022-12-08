#![cfg_attr(not(test), no_main)]
use jet_programmable_rust_binding::hostcalls::hostcall_set_outputs;
use jet_programmable_rust_binding::networking::{request, NetworkingRequest};
use serde_json::Value;

#[no_mangle]
pub fn run() {
    let request_body =
        NetworkingRequest::get("https://worldtimeapi.org/api/timezone/Asia/Shanghai".to_string());
    let respose_data = request(request_body);
    match respose_data {
        Ok(response) => {
            if let Some(body) = response.body {
                let body = String::from_utf8(body.to_vec()).unwrap();
                let body: Value = serde_json::from_str(&body).unwrap();
                match &body["datetime"] {
                    Value::String(time) => unsafe {
                        hostcall_set_outputs(time.as_ptr(), time.len());
                    },
                    _ => panic!("Return value error, the required value was not found"),
                }
            } else {
                panic!("Did not return the expected value")
            }
        }
        Err(error) => unsafe {
            hostcall_set_outputs(error.message.as_ptr(), error.message.len());
        },
    }
}
