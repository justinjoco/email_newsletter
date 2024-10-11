use email_newsletter::startup::run;
use std::net::TcpListener;
use email_newsletter::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("should read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("listener should have address binded");
    run(listener)?.await
}
