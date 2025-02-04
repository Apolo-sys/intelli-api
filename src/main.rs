use axum::Server;
use axum_server::{tls_rustls::RustlsConfig, HttpConfig};
use config::{initialize_tracing_subscriber, Database};
use dotenvy::{dotenv, var};
use hyper::Error;
use routes::service_routes;
use std::{net::TcpListener, path::PathBuf, sync::Arc};
use tracing::info;

mod config;
mod dtos;
mod entity;
mod error;
mod handlers;
mod middlewares;
mod repositories;
mod response;
mod routes;
mod services;
mod states;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    initialize_tracing_subscriber();
    let db = Database::default().await;

    let listener = TcpListener::bind(var("HOST").unwrap()).unwrap();

    info!("Server listening on {}", listener.local_addr().unwrap());
    if cfg!(debug_assertions) {
        Server::from_tcp(listener)?
            .serve(service_routes(Arc::new(db)).await)
            // .with_graceful_shutdown()
            .await?;
    } else {
        let config = RustlsConfig::from_pem_file(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("certs")
                .join("intelli.gerardz.de.crt"),
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("certs")
                .join("intelli.gerardz.de.key"),
        )
        .await
        .unwrap();

        // TODO: Check how to implement graceful shutdown
        axum_server::from_tcp_rustls(listener, config)
            .http_config(HttpConfig::new().http2_only(true).build())
            .serve(service_routes(Arc::new(db)).await)
            .await
            .unwrap();
    }

    Ok(())
}
