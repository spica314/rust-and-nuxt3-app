pub use app_api_schema::v1::{
    add_int::{V1AddIntRequest, V1AddIntResponse},
    get_int::{V1GetIntRequest, V1GetIntResponse},
};
use wasm_bindgen::prelude::*;

const API_URL: &str = match option_env!("API_URL") {
    Some(url) => url,
    None => "http://localhost:8080",
};

#[wasm_bindgen]
pub async fn v1_add_int(req: V1AddIntRequest) -> V1AddIntResponse {
    let res = gloo_net::http::Request::post(&format!("{}/v1/add-int", API_URL))
        .json(&req)
        .unwrap()
        .send()
        .await
        .unwrap();
    let res: V1AddIntResponse = res.json().await.unwrap();
    res
}

#[wasm_bindgen]
pub async fn v1_get_int(req: V1GetIntRequest) -> V1GetIntResponse {
    let res = gloo_net::http::Request::post(&format!("{}/v1/get-int", API_URL))
        .json(&req)
        .unwrap()
        .send()
        .await
        .unwrap();
    let res: V1GetIntResponse = res.json().await.unwrap();
    res
}
