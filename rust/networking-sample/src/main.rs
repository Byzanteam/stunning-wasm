#![cfg_attr(not(test), no_main)]
use jet_programmable_rust_binding::hostcalls::hostcall_set_outputs;
use jet_programmable_rust_binding::networking::{request, Get, NetworkingRequest, ResponseData};
use serde_json::Value;

#[no_mangle]
pub fn run() {
    let request_body = NetworkingRequest::Get(Get::request_body(
        "https://worldtimeapi.org/api/timezone/Asia/Shanghai".to_string(),
    ));
    let respose = request(request_body);

    match respose {
        ResponseData::Response(res) => {
            if let Some(body) = res.body {
                let body: Value = serde_json::from_str(&body).unwrap();
                match &body["datetime"] {
                    Value::String(ti) => unsafe { hostcall_set_outputs(ti.as_ptr(), ti.len()) },
                    _ => panic!("Return value error, the required value was not found"),
                };
            } else {
                panic!("Did not return the expected value")
            }
        }
        ResponseData::Message(mes) => unsafe { hostcall_set_outputs(mes.as_ptr(), mes.len()) },
    }
}
