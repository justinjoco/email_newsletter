use std::net::TcpListener;
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener should have address binded");
    let port = listener.local_addr().unwrap().port();
    let server =
        email_newsletter::startup::run(listener).expect("server should have address binded");

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
#[tokio::test]
async fn test_health_check() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("request should have executed");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn test_subscribe_returns_200_for_valid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "_name=le%20guin&_email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Should execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn test_subscribe_returns_400_for_missing_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Should execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API didn't fail with 400 bad request when the payload was {}.",
            error_message
        );
    }
}
