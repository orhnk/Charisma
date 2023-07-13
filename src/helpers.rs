use std::collections::HashMap;

use reqwest::Response;

use crate::utils::HEADERS;

pub async fn send_req(
    url: &str,
    json: &HashMap<&str, Option<&str>>,
) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let res = client
        .post(url)
        .json(json)
        .headers(HEADERS.clone()) // FIXME
        .send()
        .await?;

    Ok(res)
}
