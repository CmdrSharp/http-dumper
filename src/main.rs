use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use std::{env, net::SocketAddr, path::PathBuf};

mod router;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8080, value_parser = clap::value_parser!(u16).range(1..), env = "HTTP_DUMPER_PORT")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "http_dumper=debug,tower_http=debug");
    }

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .pretty()
        .with_file(false)
        .with_line_number(false)
        .with_level(false)
        .with_target(false)
        .init();

    let app: Router = router::create_router();

    // Start the server
    if let (Ok(cert), Ok(key)) = (
        env::var("HTTP_DUMPER_TLS_CERT"),
        env::var("HTTP_DUMPER_TLS_KEY"),
    ) {
        let cert = PathBuf::from(cert);
        let key = PathBuf::from(key);

        // If the paths don't exist
        if !cert.exists() || !key.exists() {
            tracing::error!("Certificate or key file not found");
            std::process::exit(1);
        }

        let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], args.port));
        tracing::info!(
            "Listening on {}, found certificate {:?} and key {:?}",
            addr,
            cert,
            key
        );

        let tls_config = RustlsConfig::from_pem_file(cert, key).await.unwrap();

        axum_server::bind_rustls(addr, tls_config)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    } else {
        let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], args.port));
        tracing::info!("Listening on {}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    }
}
