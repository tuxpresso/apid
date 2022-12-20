use std::net::UdpSocket;

use actix_web::{web, HttpResponse};

#[repr(C)]
#[derive(serde::Deserialize)]
pub struct FormData {
    sp: i32,
    kp: i32,
    ki: i32,
    kd: i32,
    max_temp: i32,
}

fn cast_to_bytes(data: &FormData) -> &[u8] {
    let ptr = data as *const FormData as *const u8;
    let len = std::mem::size_of::<FormData>();
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

pub async fn settings(form: web::Form<FormData>, boilerd_port: web::Data<u16>) -> HttpResponse {
    let sock = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind.");
    let dst = format!("127.0.0.1:{}", boilerd_port.as_ref());
    let bytes = cast_to_bytes(&form);
    sock.send_to(bytes, dst).expect("Failed to send.");
    HttpResponse::Ok().finish()
}
