#[tracing::instrument(name = "Handle health_check")]
pub async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().finish()
}
