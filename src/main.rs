use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = apid::configuration::get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.bind_port);
    let listener = TcpListener::bind(address).expect("Failed to bind.");
    apid::startup::run(listener)?.await
}
