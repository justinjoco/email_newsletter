use email_newsletter::configuration::get_configuration;
use email_newsletter::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("should read configuration");
    let pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("should connect to database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("listener should have address binded");
    run(listener, pool)?.await
}
