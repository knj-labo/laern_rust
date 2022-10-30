use std::sync::Arc;

use hyper_rustls::ConfigBuilderExt;
use rustls::{ClientConfig, KeyLogFile};

#[tokio::main]
async fn main() {
    let conn = hyper::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .build();

    let clinet = hyper::Client::builder().build::<_, hyper::Body>(conn);

    let response = clinet
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