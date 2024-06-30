use actix_web::{post, web};
use app_api_schema::v1::get_int::{
    V1GetIntRequest, V1GetIntResponse, V1GetIntResponseInner, V1GetIntResponseInnerOk,
};

use crate::state::AppState;

#[post("/v1/get-int")]
pub async fn v1_get_int(
    state: web::Data<AppState>,
    _req: web::Json<V1GetIntRequest>,
) -> web::Json<V1GetIntResponse> {
    let value = state.value.lock().await;

    let res = V1GetIntResponse {
        inner: V1GetIntResponseInner::Ok(V1GetIntResponseInnerOk { sum: *value }),
    };

    web::Json(res)
}
