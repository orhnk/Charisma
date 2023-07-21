use crate::{
    request::CraiyonRequest,
    utils::{MODEL_VER, URL_V1, URL_V3},
};
use clap::ValueEnum;
use std::fmt::Display;

/// Dall-e Mini API Versions
#[allow(dead_code)]
#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd, Ord, ValueEnum, Copy)]
pub enum Api {
    #[value(name = "1")]
    V1,
    // V2, // deprecated
    #[default]
    #[value(name = "3")]
    V3,
}

impl Api {
    pub fn to_response(
        &self,
        prompt: String,
        negative_prompt: String,
        model: String,
    ) -> CraiyonRequest {
        match self {
            Api::V1 => {
                // Dall-e Mini V1 is not supported anymore.
                // The breaking change is due to the fact that the API has changed on about 10th of July 2023.
                // PR's are welcome

                panic!("V1 is not supported anymore. Please use V3 instead.");

                /*
                    CraiyonRequest::V1 {
                        prompt: Some(prompt),
                    }
                */
            }

            Api::V3 => CraiyonRequest::V3 {
                prompt: Some(prompt),
                negative_prompt: Some(negative_prompt),
                model: Some(model),
                version: Some(MODEL_VER.to_string()),
                token: None, // FIXME api_tokens are not supported yet.
            },
        }
    }
}

impl AsRef<str> for Api {
    fn as_ref(&self) -> &str {
        match self {
            Api::V1 => URL_V1,
            // Api::V2 => URL_V2,
            Api::V3 => URL_V3,
        }
    }
}

impl Display for Api {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match self {
            Api::V1 => f.write_str(URL_V1),
            // Api::V2 => f.write_str(URL_V2),
            Api::V3 => f.write_str(URL_V3),
        }
    }
}
