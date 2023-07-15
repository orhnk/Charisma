use serde::{Deserialize, Serialize};

/// Response Deserializer
#[derive(Debug, Serialize, Deserialize)]
pub struct CraiyonResponse {
    pub images: Vec<String>,
}

//// Request Serializer for api V1
#[derive(Debug, Serialize, Deserialize)]
pub struct CraiyonRequestV3<'a> {
    pub prompt: Option<&'a str>,
    pub negative_prompt: Option<&'a str>,
    pub model: Option<&'a str>,
    pub token: Option<&'a str>,
    pub version: Option<&'a str>,
}

// Request Serializer for api V1
#[derive(Debug, Serialize, Deserialize)]
pub struct CraiyonRequestV1<'a> {
    pub prompt: Option<&'a str>,
}
