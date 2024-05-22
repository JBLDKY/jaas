use jaas::configuration::get_configuration;
use jaas::startup::Application;
use jaas::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("jaas".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration.");
    let app = Application::build(config).await?;

    app.run_until_stopped().await?;
    Ok(())
}
