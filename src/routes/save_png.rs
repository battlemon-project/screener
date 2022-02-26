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

    let bytes = element
        .screenshot_as_png()
        .await
        .expect("Couldn't click on element");

    let file_path = "/home/seluser/screenshots/screen.png".to_string();
    let mut file = tokio::fs::File::create(file_path)
        .await
        .expect("Couldn't create file");
    file.write_all(&bytes)
        .await
        .expect("Couldn't write bytes to file");

    HttpResponse::Ok().finish()
}
