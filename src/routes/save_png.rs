use std::io::Cursor;
use std::time::Duration;

use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse};
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use thirtyfour::{By, WebDriver};
use tokio::time::sleep;

use crate::config::get_config;

pub async fn save_png(driver: web::Data<WebDriver>, ipfs: web::Data<IpfsClient>) -> HttpResponse {
    let constructor_url = &get_config().await.constructor.url();
    driver.get(constructor_url).await.expect("Couldn't get url");

    while driver.find_element(By::Id("download")).await.is_err() {
        sleep(Duration::from_millis(100)).await;
    }

    let canvas = driver
        .find_element(By::ClassName("threejs-container"))
        .await
        .expect("Couldn't find element");
    let screenshot = canvas
        .screenshot_as_png()
        .await
        .expect("Couldn't capture screenshot");
    let data = Cursor::new(screenshot);
    let res = ipfs.add(data).await.expect("Couldn't add to ipfs data");
    let json = serde_json::json!({
        "hash": res.hash,
    });

    HttpResponse::Ok().json(json)
}
