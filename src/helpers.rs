use crate::utils::HEADERS;
use serde::Serialize;

pub async fn send_req<T>(
    url: &str,
    json: &T,
) -> Result<reqwest::Response, reqwest::Error>
where
    T: Serialize + ?Sized,
{
    let client = reqwest::Client::new();

    let res = client
        .post(url)
        .json(json)
        .headers(HEADERS.clone()) // FIXME
        .send()
        .await;

    res
}
