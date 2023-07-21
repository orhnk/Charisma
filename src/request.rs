use serde::{Deserialize, Serialize};

/// Request Serializer
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CraiyonRequest {
    V1 {
        prompt: Option<String>,
    },

    V3 {
        prompt: Option<String>,
        negative_prompt: Option<String>,
        model: Option<String>,
        token: Option<String>,
        version: Option<String>,
    },
}

/// Response Serializer
#[derive(Debug, Serialize, Deserialize)]
pub struct CraiyonResponse {
    pub images: Vec<String>,
}
