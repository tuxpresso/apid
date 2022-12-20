use actix_web::HttpResponse;

pub async fn settings() -> HttpResponse {
    HttpResponse::Ok().finish()
}
