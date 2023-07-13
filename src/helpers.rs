use reqwest::Response;
use serde::Serialize;

use crate::utils::HEADERS;

pub async fn send_req<T>(
    url: &str,
    json: &T,
) -> Result<Response, reqwest::Error>
where
    T: Serialize + ?Sized,
{
    let client = reqwest::Client::new();

    let res = client
        .post(url)
        .json(json)
        .headers(HEADERS.clone()) // FIXME
        .send()
        .await?;

    Ok(res)
}
