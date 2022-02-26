use std::path::Path;

use actix_web::{web, HttpResponse};
use thirtyfour::{By, WebDriver};
use tokio::io::AsyncWriteExt;

pub async fn save_png(driver: web::Data<WebDriver>) -> HttpResponse {
    driver
        .get("http://constructor")
        .await
        .expect("Couldn't get url");

    let element = driver
        .find_element(By::ClassName("threejs-container"))
        .await
        .expect("Couldn't find element");

    let file_path = Path::new("/app/screenshots/screen.png");
    element
        .screenshot(file_path)
        .await
        .expect("Couldn't save screenshot");

    HttpResponse::Ok().finish()
}
