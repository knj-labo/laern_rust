#[tokio::main]
async fn main() {
    let response = reqwest::get("http://example.org").await.unwrap();
    println!(
        "Got HTTP {}, with headers: {:#?}",
        response.status(),
        response.headers()
    );
}
