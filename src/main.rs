use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use std::env;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let domain = env::var("DOMAIN").expect("DOMAIN NOT SPECIFIED");
    let site_directory = env::var("SITE_DIRECTORY").expect("SITE_DIRECTORY NOT SPECIFIED");

    let config = RustlsConfig::from_pem_file(
        PathBuf::from(format!("/etc/letsencrypt/live/{}/fullchain.pem", domain)),
        PathBuf::from(format!("/etc/letsencrypt/live/{}/privkey.pem", domain)),
    )
    .await
    .unwrap();

    let app = Router::new().nest_service("/", ServeDir::new(site_directory));
    // run https server
    let addr = SocketAddr::from(([0, 0, 0, 0], 443));
    tracing::info!("listening on {}", addr);
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
