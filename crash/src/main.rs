#[tokio::main]
async fn main() {
    let response = hyper::Client::new()
        .get("http://example.org".parse().unwrap())
        .await
        .unwrap();

    // String::from_utf8 can fail because the body might not actually be valid UTF-8, it could just be arbitrary binary spaghetti.
    let body = String::from_utf8(
        hyper::body::to_bytes(response.into_body())
            .await
            .unwrap()
            .to_vec(),
    )
        .unwrap();
    println!("response body: {body}");
}