use zero2prod::configuration::get_configuration;
use zero2prod::startup::*;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Redirect all logs events to our subscriber
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration file");
    let application = Application::build(configuration)?;
    application.run_until_stopped().await?;
    Ok(())
}
