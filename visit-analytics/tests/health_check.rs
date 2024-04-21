use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to get response");

    assert!(response.status().is_success());

    let response_text = response.text().await.expect("Failed to get payload");

    assert_eq!(response_text, "It works!");
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrieve the port assigned to app by the OS
    let port = listener.local_addr().unwrap().port();
    let server = lokate_visit_analytics::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
