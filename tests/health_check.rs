use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let addr = "127.0.0.1";
    let listener =
        TcpListener::bind(&format!("{}:0", addr)).expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port();
    let server = ztplib::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("{}:{}", addr, port)
}
