

#[get("/client/status")]
pub async fn status() -> &'static str {
    "Status"
}