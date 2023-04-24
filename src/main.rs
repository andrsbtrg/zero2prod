use std::net::TcpListener;

use zero2prod::run;

const ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let lst = TcpListener::bind(ADDRESS).unwrap();
    run(lst)?.await
}
