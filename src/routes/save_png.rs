use actix_web::{web, HttpResponse};
use thirtyfour::{By, WebDriver};

pub async fn save_png(driver: web::Data<WebDriver>) -> HttpResponse {
    driver
        .get("http://constructor")
        .await
        .expect("Couldn't get url");

    let element = driver
        .find_element(By::Id("download"))
        .await
        .expect("Couldn't find element");

    element.click().await.expect("Couldn't click on element");

    HttpResponse::Ok().finish()
}
