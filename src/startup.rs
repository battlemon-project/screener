use std::net::TcpListener;

use actix_web::web;
use ipfs_api_backend_hyper::IpfsClient;
use thirtyfour::WebDriver;

use crate::{config, routes, Result};

#[tracing::instrument(name = "Running actix-web service", skip(listener, web_driver, ipfs))]
pub fn run(
    listener: TcpListener,
    web_driver: WebDriver,
    ipfs: IpfsClient,
) -> std::io::Result<actix_web::dev::Server> {
    let web_driver = web::Data::new(web_driver);
    let ipfs = web::Data::new(ipfs);

    tracing::info!("Running actix-web server");
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/save_png", web::get().to(routes::get_png_and_push_ipfs))
            .app_data(web_driver.clone())
            .app_data(ipfs.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub async fn connect_web_driver(address: &str, headless: bool) -> Result<WebDriver> {
    let browser_settings = config::set_up_browser()?;
    let mut capabilities = thirtyfour::DesiredCapabilities::firefox();
    capabilities.set_preferences(browser_settings)?;
    if headless {
        capabilities.set_headless()?;
    }
    let driver = thirtyfour::WebDriver::new(address, &capabilities).await?;

    Ok(driver)
}
