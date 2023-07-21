use crate::utils::HEADERS;
use serde::Serialize;

#[inline(always)]
pub async fn send_req<T>(url: &str, json: &T) -> Result<reqwest::Response, reqwest::Error>
where
    T: Serialize + ?Sized,
{
    // Here are some reasons that this function creates a new client in each call:
    //
    // https://github.com/seanmonstar/reqwest/issues/600#issuecomment-523979889
    // https://docs.rs/reqwest/0.10.9/reqwest/struct.Client.html
    // https://stackoverflow.com/questions/69384070/cannot-move-out-of-an-arc
    //
    // Note: The code uses this helper only once in each task (think as a thread)

    let client = reqwest::Client::new();

    client
        .post(url)
        .json(json)
        .headers(HEADERS.clone()) // FIXME
        .send()
        .await
}
