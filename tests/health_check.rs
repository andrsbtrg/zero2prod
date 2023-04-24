use std::net::TcpListener;

fn spawn_app() -> String {
    let lst = TcpListener::bind("127.0.0.1:0").unwrap();

    // retrieve the port assigned by the OS
    let port = lst.local_addr().unwrap().port();

    let _ = tokio::spawn(zero2prod::run(lst).expect("Failed to bind radom port."));
    format!("127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let endpoint = format!("http://{}/health_check", address);

    let client = reqwest::Client::new();

    let response = client
        .get(endpoint)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
