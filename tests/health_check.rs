use std::net::TcpListener;
use std::net::UdpSocket;
use std::time::Duration;

pub struct TestApp {
    pub address: String,
    pub boilerd_socket: UdpSocket,
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn settings_returns_a_200_for_valid_form_data() {
    let app = spawn_app();
    let client = reqwest::Client::new();

    let body = "sp=512&kp=64&ki=1&kd=0&max_temp=1024";
    let response = client
        .post(&format!("{}/settings", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    let mut buf = [0; 1024];
    let (n, addr) = app
        .boilerd_socket
        .recv_from(&mut buf)
        .expect("Failed to recv.");
    println!("{} bytes response from {:?}", n, addr);

    assert_eq!(200, response.status().as_u16());
    todo!()
}

fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let listener_port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", listener_port);

    let boilerd_socket =
        UdpSocket::bind("127.0.0.1:0").expect("failed to bind mock boilerd address.");
    boilerd_socket
        .set_read_timeout(Some(Duration::new(1, 0)))
        .expect("Failed to set udp read timeout.");
    let boilerd_port = boilerd_socket.local_addr().unwrap().port();

    let server = apid::startup::run(listener, boilerd_port).expect("Failed to bind address.");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        boilerd_socket,
    }
}
