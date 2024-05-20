use jaas::configuration::{get_configuration, Environment};
use jaas::email_client::EmailClient;
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

    let environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let pool = match environment {
        Environment::Production => {
            // FLY IO just kinda gives u the URL so ¯\_(ツ)_/¯
            PgPool::connect_lazy(&std::env::var("DATABASE_URL").unwrap()).unwrap()
        }
        Environment::Local => PgPool::connect_lazy_with(config.database.with_db()),
    };

    let sender_email = config
        .email_client
        .sender()
        .expect("Invalid sender email address");

    let timeout = config.email_client.timeout();
    let email_client = EmailClient::new(
        config.email_client.base_url,
        sender_email,
        config.email_client.authorization_token,
        timeout,
    );

    let listener = TcpListener::bind(address).expect("Failed to bind to random port");

    run(listener, pool, email_client)?.await?;

    Ok(())
}
