#[tokio::main]
async fn main() {
    let response = hyper::Client::new().get("http://example.org".parse().unwrap())
        .await
        .unwrap();
    println!(
        "Got HTTP {}, with headers: {:#?}",
        response.status(),
        response.headers()
    );

    let body = response.body();
    println!("Body: {:?}", body);
}
