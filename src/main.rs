mod network;
mod protocol;

use network::setup_listener;
use opentelemetry::{global, runtime::TokioCurrentThread};
use std::error::Error;
use tracing_subscriber::fmt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_telemetry()?;
    setup_listener().await?;

    loop {
        std::thread::sleep(std::time::Duration::new(5, 0));
    }
    // Ok(())
}

fn setup_telemetry() -> Result<(), Box<dyn Error>> {
    let subscriber = tracing_subscriber::registry().with(fmt::Layer::default());
    let address = std::env::var("JAEGER_ADDRESS").unwrap_or_else(|e| {
        println!("{}", e.to_string());
        "".to_owned()
    });

    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("ironmc")
        .with_endpoint(address)
        .with_auto_split_batch(true)
        .install_batch(TokioCurrentThread);

    if tracer.is_ok() {
        // If dial to jaeger succeeded, add opentelemetry
        println!("Connected to Jaeger, adding telemetry");
        let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer.unwrap());

        subscriber.with(opentelemetry).try_init()?;
    } else {
        // Else just init tracing without telemetry
        println!("Could not connect to Jaeger, skipping telemetry");
        subscriber.try_init()?;
    }

    Ok(())
}
