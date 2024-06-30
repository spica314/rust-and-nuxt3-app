use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V1GetIntRequest {}

#[wasm_bindgen]
impl V1GetIntRequest {
    #[wasm_bindgen]
    pub fn new() -> V1GetIntRequest {
        V1GetIntRequest {}
    }
}

impl Default for V1GetIntRequest {
    fn default() -> Self {
        V1GetIntRequest::new()
    }
}

// Since non-C-style enums cannot be directly handled with wasm_bindgen,
// here we create functions to wrap them.
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V1GetIntResponse {
    #[wasm_bindgen(skip)]
    pub inner: V1GetIntResponseInner,
}

#[wasm_bindgen]
impl V1GetIntResponse {
    #[wasm_bindgen]
    pub fn is_ok(&self) -> bool {
        match &self.inner {
            V1GetIntResponseInner::Ok(_) => true,
            V1GetIntResponseInner::Err(_) => false,
        }
    }

    #[wasm_bindgen]
    pub fn is_err(&self) -> bool {
        match &self.inner {
            V1GetIntResponseInner::Ok(_) => false,
            V1GetIntResponseInner::Err(_) => true,
        }
    }

    #[wasm_bindgen]
    pub fn ok(&self) -> Option<V1GetIntResponseInnerOk> {
        match &self.inner {
            V1GetIntResponseInner::Ok(inner) => Some(inner.clone()),
            V1GetIntResponseInner::Err(_) => None,
        }
    }

    #[wasm_bindgen]
    pub fn err(&self) -> Option<V1GetIntResponseInnerErr> {
        match &self.inner {
            V1GetIntResponseInner::Ok(_) => None,
            V1GetIntResponseInner::Err(inner) => Some(inner.clone()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum V1GetIntResponseInner {
    Ok(V1GetIntResponseInnerOk),
    Err(V1GetIntResponseInnerErr),
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V1GetIntResponseInnerOk {
    pub sum: i32,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V1GetIntResponseInnerErr {
    pub error: String,
}
