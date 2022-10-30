use futures::TryStreamExt;

#[tokio::main]
async fn main() {
    let response = hyper::Client::new()
        .get("http://example.org".parse().unwrap())
        .await
        .unwrap();

    let mut body = response.into_body();

    // 1度に1つのバッファを取得しながらデータを polling することができる。
    while let Some(buffer) = body.try_next().await.unwrap() {
        println!("Read {} bytes", buffer.len());
    }
}
