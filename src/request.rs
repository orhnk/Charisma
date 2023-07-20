use serde::{Deserialize, Serialize};

/// Request Serializer for api V1
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

/// Response Deserializer
#[derive(Debug, Serialize, Deserialize)]
pub struct CraiyonResponse {
    pub images: Vec<String>,
}
