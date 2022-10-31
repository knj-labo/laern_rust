use std::{str::FromStr, sync::Arc};

use hyper::{Client, Request};
use rustls::{Certificate, ClientConfig, KeyLogFile, RootCertStore};
use tracing::info;
use tracing_subscriber::{filter::targets::Targets, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    let filter_layer =
        Targets::from_str(std::env::var("RUST_LOG").as_deref().unwrap_or("info")).unwrap();
    let format_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(format_layer)
        .init();

    info!("Setting up TLS");
    let mut root_store = RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs()? {
        root_store.add(&Certificate(cert.0))?;
    }

    let mut client_config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    client_config.key_log = Arc::new(KeyLogFile::new());

    let connector = tokio_rustls::TlsConnector::from(Arc::new(client_config));

    let addr = "example.org::443"
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| eyre!("Failed to resolve address for example.org:443"))?;

    info!("Establishing TCP connection...");
    let stream = TcpStream::connect(addr).await?;

    info!("Establishing TLS session...");
    let stream = connector.connect("example.org".try_into()?, stream).await?;

    info!("Establishing HTTP/2 connection...");
    let (_send_req, conn) = h2::client::handshake(stream).await?;
    tokio::spawn(conn);

    info!("Now what?");

    Ok(())
}