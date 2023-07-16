use crate::utils::{URL_V1, URL_V3};
use clap::ValueEnum;
use std::fmt::Display;

/// API Versions for craiyon.com
#[allow(dead_code)]
#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd, Ord, ValueEnum)]
pub enum Api {
    #[value(name = "1")]
    V1,
    // V2, // deprecated
    #[default]
    #[value(name = "3")]
    V3,
}

impl Api {
    pub fn as_str(&self) -> &str {
        match self {
            Api::V1 => URL_V1,
            // Api::V2 => f.write_str(URL_V2),
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
