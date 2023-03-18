use log::debug;
use serde::de;

pub async fn get_http_result<
    'a,
    'b,
    T: de::DeserializeOwned + std::clone::Clone,
    U: de::DeserializeOwned + serde::Serialize + std::clone::Clone,
>(
    url: String,
    request: U,
) -> T {
    let request_json = serde_json::to_string(&request).unwrap();
    let result_json: String = send_http_message(request_json, url).await;
    debug!("{result_json:?}");
    let value: T = serde_json::from_str(&result_json).unwrap();
    value
}

pub async fn send_http_message(json_message: String, url: String) -> String {
    let client = reqwest::Client::new();

    debug!("{json_message}");
    debug!("{url}");

    return client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json_message)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
}
