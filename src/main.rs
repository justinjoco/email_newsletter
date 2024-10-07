use std::net::TcpListener;
use email_newsletter::startup::run;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener should have address binded");
    run(listener)?.await
}
