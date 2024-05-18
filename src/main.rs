use jaas::configuration::get_configuration;
use jaas::startup::run;
use jaas::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("jaas".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration.");
    let address = format!("{}:{}", config.application.host, config.application.port);

    let pool = PgPool::connect_lazy_with(config.database.with_db());

    let listener = TcpListener::bind(address).expect("Failed to bind to random port");

    run(listener, pool)?.await
}
