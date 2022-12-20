use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    sp: i32,
    kp: i32,
    ki: i32,
    kd: i32,
    max_temp: i32,
}

pub async fn settings(form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
