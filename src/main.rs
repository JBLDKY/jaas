use jaas::configuration::get_configuration;
use jaas::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = TcpListener::bind(address).expect("Failed to bind to random port");

    run(listener)?.await
}
