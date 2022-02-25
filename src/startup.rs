use std::net::TcpListener;

use actix_web::web;
use thirtyfour::WebDriver;

use crate::{config, routes, Result};

pub fn run(
    listener: TcpListener,
    web_driver: WebDriver,
) -> std::io::Result<actix_web::dev::Server> {
    let web_driver = web::Data::new(web_driver);
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/save_png", web::get().to(routes::save_png))
            .app_data(web_driver.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub async fn get_web_driver(address: &str) -> Result<WebDriver> {
    let browser_settings = config::get_browser_settings()?;
    let mut capabilities = thirtyfour::DesiredCapabilities::firefox();
    capabilities.set_preferences(browser_settings)?;
    let driver = thirtyfour::WebDriver::new(address, &capabilities).await?;

    Ok(driver)
}
