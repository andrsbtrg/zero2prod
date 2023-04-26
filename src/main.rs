use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to load configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let lst = TcpListener::bind(address).unwrap();

    run(lst)?.await
}
