use std::net::TcpListener;

use battlemon_screener::{config, startup};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = config::get_config().expect("Couldn't get config");
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address).expect("Couldn't bind address");
    let remote_webdriver_address =
        format!("{}:{}", config.webdriver.address, config.webdriver.port);
    let web_driver = startup::get_web_driver(&remote_webdriver_address)
        .await
        .expect("Couldn't get web driver");
    startup::run(listener, web_driver)?.await
}
