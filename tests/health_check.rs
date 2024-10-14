use email_newsletter::configuration::{get_configuration, DatabaseSettings};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}
async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener should have address binded");
    let port = listener.local_addr().unwrap().port();

    let address = format!("http://127.0.0.1:{}", port);
    let mut configuration = get_configuration().expect("configuration should be valid");
    configuration.database.name = Uuid::new_v4().to_string();
    let pool = configure_db(&configuration.database).await;
    let server = email_newsletter::startup::run(listener, pool.clone())
        .expect("server should have address binded");

    let _ = tokio::spawn(server);

    println!("App address {}", &address);

    TestApp { address, pool }
}

async fn configure_db(config: &DatabaseSettings) -> PgPool {
    let settings = DatabaseSettings {
        name: "newsletter".to_string(),
        username: "app".to_string(),
        password: "secret".to_string(),
        ..config.clone()
    };

    let mut connection = PgConnection::connect(&settings.connection_string())
        .await
        .expect("should have connected to db");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
        .await
        .expect("db should have been created");

    let pool = PgPool::connect(&config.connection_string())
        .await
        .expect("pool should have been created");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrations should have occurred");

    pool
}
#[tokio::test]
async fn test_health_check() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    println!("{}", &app.address);
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("request should have executed");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn test_subscribe_returns_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Should execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT name, email FROM subscriptions")
        .fetch_one(&app.pool)
        .await
        .expect("Should be executed");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn test_subscribe_returns_400_for_missing_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
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
