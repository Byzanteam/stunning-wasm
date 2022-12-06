use jet_programmable_rust_binding::request::{network, RequestData, ResponseData};

#[no_mangle]
pub fn run() {
    let request_data = RequestData::new(
        "GET".to_string(),
        "https://baidu.com".to_string(),
        Vec::new(),
        None,
    );
    let _response = {
        let response_data: ResponseData = serde_json::from_str(network(request_data)).unwrap();
        response_data.print_response()
    };
}
