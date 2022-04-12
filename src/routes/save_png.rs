use std::io::Cursor;
use std::time::Duration;

use actix_web::{HttpRequest, HttpResponse, web};
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use thirtyfour::By;
use tokio::time::sleep;

use crate::config::get_config;
use crate::startup;

#[tracing::instrument(name = "Handling save_png", skip(ipfs))]
pub async fn get_png_and_push_ipfs(req: HttpRequest, ipfs: web::Data<IpfsClient>) -> HttpResponse {
    let constructor_url = &get_config().await.constructor.url();
    let url = format!("{}?{}", constructor_url, req.query_string());
    tracing::info!("Url: {}", url);

    let config = get_config().await;
    let web_driver =
        startup::connect_web_driver(&config.webdriver.url(), config.webdriver.headless())
            .await
            .expect("Couldn't connect to webdriver");

    web_driver.get(url).await.expect("Couldn't get url");

    tracing::info!("Finding element with id download.");
    while web_driver.find_element(By::Id("download")).await.is_err() {
        sleep(Duration::from_millis(100)).await;
    }

    tracing::info!("Getting picture's bytes");
    let canvas = web_driver
        .find_element(By::ClassName("threejs-container"))
        .await
        .expect("Couldn't find element");
    let screenshot = canvas
        .screenshot_as_png()
        .await
        .expect("Couldn't capture screenshot");

    web_driver.quit().await.expect("Couldn't quit webdriver");

    let data = Cursor::new(screenshot);

    tracing::info!("Adding the picture to IPFS");
    let res = ipfs.add(data).await.expect("Couldn't add to ipfs data");
    let json = serde_json::json!({
        "hash": res.hash,
    });

    HttpResponse::Ok().json(json)
}
