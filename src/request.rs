use serde::{Deserialize, Serialize};

/// Request Serializer for api V1
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CraiyonRequest<'a> {
    V1 {
        prompt: Option<&'a str>,
    },

    V3 {
        prompt: Option<&'a str>,
        negative_prompt: Option<&'a str>,
        model: Option<&'a str>,
        token: Option<&'a str>,
        version: Option<&'a str>,
    },
}

/// Response Deserializer
#[derive(Debug, Serialize, Deserialize)]
pub struct CraiyonResponse {
    pub images: Vec<String>,
}
