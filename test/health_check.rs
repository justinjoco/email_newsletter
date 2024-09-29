#[tokio::test]
async fn test_health_check() {
    spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("request should have executed");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() -> Result<(), std::io::Error>{
    let server = zero2prod::run().expect("server should have address binded");

    let _ = tokio::spawn(server);
}