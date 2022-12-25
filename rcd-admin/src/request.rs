use gloo::net::http::{Request, Method};
use yew::{platform::spawn_local, AttrValue, Callback};

/// sends an HTTP POST to the specified URL with the rcd-message as JSON, returning JSON
pub fn get_data(url: String, body: String, callback: Callback<AttrValue>) {
    spawn_local(async move {
        let http_response = Request::new(&url)
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        callback.emit(AttrValue::from(http_response));
    });
}
