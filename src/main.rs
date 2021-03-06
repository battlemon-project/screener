use ipfs_api_backend_hyper::TryFromUri;

use battlemon_screener::{config, startup, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("battlemon_screener".into(), "info".into());
    telemetry::init_subscriber(subscriber);
    let config = config::get_config().await;
    tracing::info!("Bind listener.");
    let listener =
        std::net::TcpListener::bind(config.application.address()).expect("Couldn't bind address");

    tracing::info!("Connect to IPFS.");
    let ipfs = ipfs_api_backend_hyper::IpfsClient::from_str(&config.ipfs.url())
        .expect("Couldn't connect to IPFS");

    startup::run(listener, ipfs)?.await
}
