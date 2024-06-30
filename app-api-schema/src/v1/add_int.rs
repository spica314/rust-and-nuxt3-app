use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V1AddIntRequest {
    pub diff: i32,
}

#[wasm_bindgen]
impl V1AddIntRequest {
    #[wasm_bindgen]
    pub fn new(diff: i32) -> V1AddIntRequest {
        V1AddIntRequest { diff }
    }
}

// Since non-C-style enums cannot be directly handled with wasm_bindgen,
// here we create functions to wrap them.
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V1AddIntResponse {
    #[wasm_bindgen(skip)]
    pub inner: V1AddIntResponseInner,
}

#[wasm_bindgen]
impl V1AddIntResponse {
    #[wasm_bindgen]
    pub fn is_ok(&self) -> bool {
        match &self.inner {
            V1AddIntResponseInner::Ok(_) => true,
            V1AddIntResponseInner::Err(_) => false,
        }
    }

    #[wasm_bindgen]
    pub fn is_err(&self) -> bool {
        match &self.inner {
            V1AddIntResponseInner::Ok(_) => false,
            V1AddIntResponseInner::Err(_) => true,
        }
    }

    #[wasm_bindgen]
    pub fn ok(&self) -> Option<V1AddIntResponseInnerOk> {
        match &self.inner {
            V1AddIntResponseInner::Ok(inner) => Some(inner.clone()),
            V1AddIntResponseInner::Err(_) => None,
        }
    }

    #[wasm_bindgen]
    pub fn err(&self) -> Option<V1AddIntResponseInnerErr> {
        match &self.inner {
            V1AddIntResponseInner::Ok(_) => None,
            V1AddIntResponseInner::Err(inner) => Some(inner.clone()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum V1AddIntResponseInner {
    Ok(V1AddIntResponseInnerOk),
    Err(V1AddIntResponseInnerErr),
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V1AddIntResponseInnerOk {
    pub sum: i32,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V1AddIntResponseInnerErr {
    pub message: String,
}
