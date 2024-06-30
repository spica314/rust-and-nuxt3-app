use actix_web::{post, web};
use app_api_schema::v1::add_int::{
    V1AddIntRequest, V1AddIntResponse, V1AddIntResponseInner, V1AddIntResponseInnerErr,
    V1AddIntResponseInnerOk,
};

use crate::state::AppState;

#[post("/v1/add-int")]
pub async fn v1_add_int(
    state: web::Data<AppState>,
    req: web::Json<V1AddIntRequest>,
) -> web::Json<V1AddIntResponse> {
    let mut value = state.value.lock().await;

    if *value + req.diff < 0 {
        let res = V1AddIntResponse {
            inner: V1AddIntResponseInner::Err(V1AddIntResponseInnerErr {
                message: "Value would be negative".to_string(),
            }),
        };
        return web::Json(res);
    }

    *value += req.diff;

    let res = V1AddIntResponse {
        inner: V1AddIntResponseInner::Ok(V1AddIntResponseInnerOk { sum: *value }),
    };

    web::Json(res)
}
