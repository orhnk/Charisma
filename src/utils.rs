use lazy_static::lazy_static;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

lazy_static! {
    pub static ref HEADERS: HeaderMap = {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    };
}

pub const URL_V3: &str = "https://api.craiyon.com/v3";
// const URL_V2: &str = "https://api.craiyon.com/draw"; // deprecated
pub const URL_V1: &str = "https://backend.craiyon.com/generate";
pub const URL_IMAGE: &str = "https://img.craiyon.com";
pub const MODEL_VER: &str = "35s5hfwn9n78gb06";

pub const IMAGE_PER_REQUEST: usize = 9;
