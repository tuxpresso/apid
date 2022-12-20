use std::net::UdpSocket;

use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    sp: i32,
    kp: i32,
    ki: i32,
    kd: i32,
    max_temp: i32,
}

pub async fn settings(_form: web::Form<FormData>, boilerd_port: web::Data<u16>) -> HttpResponse {
    let sock = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind.");
    let dst = format!("127.0.0.1:{}", boilerd_port.as_ref());
    sock.send_to(&[0; 10], dst).expect("Failed to send.");
    HttpResponse::Ok().finish()
}
