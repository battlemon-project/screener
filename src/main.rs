use ipfs_api_backend_hyper::TryFromUri;

use battlemon_screener::{config, startup, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("battlemon_indexer".into(), "info".into());
    telemetry::init_subscriber(subscriber);
    let config = config::get_config().await;
    let listener =
        std::net::TcpListener::bind(config.application.address()).expect("Couldn't bind address");
    let web_driver = startup::get_web_driver(&config.webdriver.url(), config.webdriver.headless())
        .await
        .expect("Couldn't get web driver");
    let ipfs = ipfs_api_backend_hyper::IpfsClient::from_str(&config.ipfs.url())
        .expect("Couldn't connect to IPFS");

    startup::run(listener, web_driver, ipfs)?.await
}
