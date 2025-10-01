pub fn build_request(method: &str, params: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 0
    })
}

#[macro_export]
macro_rules! rpc_request {
    ($method:expr, $($param:tt),* $(,)?) => {
        $crate::json::build_request($method, serde_json::json!([$($param),*]))
    };
}