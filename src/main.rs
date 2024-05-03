use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::*;



#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Redirect all logs events to our subscriber
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration file");
    let connection = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
