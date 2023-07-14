use serde::{Serialize, Deserialize};

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

//HashMap::from([
//                ("prompt", Some(prompt)),
//                ("negative_prompt", Some(negative_prompt)),
//                ("model", Some(model)),
//                ("token", self.api_token),
//                ("version", Some(MODEL_VER)),
//            ]),

//// Response Serializer and Deserializer for V3
// #[derive(Debug, Serialize, Deserialize)]
// pub enum CraiyonRequest<'a> {
//     V1 {
//         prompt: Option<&'a str>,
//     },
//
//     V3 {
//         prompt: Option<&'a str>,
//         negative_prompt: Option<&'a str>,
//         model: Option<&'a str>,
//         token: Option<&'a str>,
//         version: Option<&'a str>,
//     },
// }
