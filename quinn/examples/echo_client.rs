//! This example demonstrates how to make a QUIC connection that ignores the server certificate.
//!
//! can be used with echo_server
//!
//! note: requires special configuration in Cargo.toml

use quinn::VarInt;
use std::{error::Error, net::SocketAddr, sync::Arc};
use std::ops::Range;
use tracing::info;
use quinn::{ClientConfig, Endpoint};

mod common;
use common::make_server_endpoint;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:5000".parse().unwrap();
    run_client(addr).await?;
    Ok(())
}

/// Runs a QUIC server bound to given address.
async fn run_server(addr: SocketAddr) {
    let (endpoint, _server_cert) = make_server_endpoint(addr).unwrap();
    // accept a single connection
    let incoming_conn = endpoint.accept().await.unwrap();
    let conn = incoming_conn.await.unwrap();
    println!(
        "[server] connection accepted: addr={}",
        conn.remote_address()
    );
}

async fn run_client(server_addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let client_cfg = configure_client();
    let mut endpoint = Endpoint::client("127.0.0.1:0".parse().unwrap())?;
    endpoint.set_default_client_config(client_cfg);

    // connect to server
    let connection = endpoint
        .connect(server_addr, "localhost")
        .unwrap()
        .await
        .unwrap();
    println!("[client] connected: addr={}", connection.remote_address());

    // let mut streams = vec![];

    connection.set_max_concurrent_bi_streams(VarInt::from_u32(3));

    let (mut send, recv) = connection.open_bi().await.unwrap();
    send.finish().await.unwrap();
    let (mut send, recv) = connection.open_bi().await.unwrap();
    send.finish().await.unwrap();
    let (mut send, recv) = connection.open_bi().await.unwrap();
    send.finish().await.unwrap();
    let (mut send, recv) = connection.open_bi().await.unwrap();
    send.finish().await.unwrap();
    let (mut send, recv) = connection.open_bi().await.unwrap();
    send.finish().await.unwrap();
    let (mut send, recv) = connection.open_bi().await.unwrap();
    send.finish().await.unwrap();
    let (mut send, recv) = connection.open_bi().await.unwrap();
    send.finish().await.unwrap();
    let (mut send, recv) = connection.open_bi().await.unwrap();
    send.finish().await.unwrap();
    let (mut send, recv) = connection.open_bi().await.unwrap();
    send.finish().await.unwrap();


    // for _ in 0..10 {
    //     let result = connection.open_bi().await;
    //     println!("opened : {:?}", result);
    //     let (mut send, recv) = result.unwrap();
    //     streams.push((send, recv));
    // }

    // for (ref mut send, ref recv) in &mut streams {
    //     send.write_all(b"hello world").await?;
    //     println!("written data to server");
    // }
    //
    // for (ref mut send, recv) in streams {
    //     send.finish().await?;
    //     let response = recv.read_to_end(10*1024).await.unwrap();
    //     println!("response size {}", response.len());
    // }

    // Dropping handles allows the corresponding objects to automatically shut down
    drop(connection);

    Ok(())
}

/// Dummy certificate verifier that treats any certificate as valid.
/// NOTE, such verification is vulnerable to MITM attacks, but convenient for testing.
struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

fn configure_client() -> ClientConfig {
    let crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(SkipServerVerification::new())
        .with_no_client_auth();

    ClientConfig::new(Arc::new(crypto))
}
