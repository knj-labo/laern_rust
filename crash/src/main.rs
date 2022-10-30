use std::{str::FromStr, sync::Arc};

use color_eyre::eyre::eyre;
use nom::Offset;
use rustls::{Certificate, ClientConfig, KeyLogFile, RootCertStore};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::info;
use tracing_subscriber::{filter::targets::Targets, layer::SubscriberExt, util::SubscriberInitExt};

mod http11;

#[tokio::main]
async fn main() {
}