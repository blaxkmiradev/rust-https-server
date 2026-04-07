use axum::{routing::get, Router};
use std::{fs::File, io::BufReader, net::SocketAddr};
use tokio_rustls::TlsAcceptor;
use rustls::{ServerConfig, pki_types::PrivateKeyDer};

async fn hello() -> &'static str {
    "Hello HTTPS from Rust 🚀"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

    let cert_file = &mut BufReader::new(File::open("certs/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("certs/key.pem").unwrap());

    let certs = rustls_pemfile::certs(cert_file)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let key = rustls_pemfile::private_key(key_file)
        .unwrap()
        .unwrap();

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, PrivateKeyDer::Pkcs8(key))
        .unwrap();

    let acceptor = TlsAcceptor::from(std::sync::Arc::new(config));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("HTTPS running at https://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let app = app.clone();
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            let tls_stream = acceptor.accept(stream).await.unwrap();

            axum::serve(
                tokio::io::BufReader::new(tls_stream),
                app,
            ).await.unwrap();
        });
    }
      }
