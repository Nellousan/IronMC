mod connection;

use std::error::Error;
use tokio::net::TcpListener;
use tracing::{error, warn};

// static mut CONNECTIONS: RwLock<HashMap<Uuid, Connection>> = RwLock::new(HashMap::new());

// Sets up the listener and spawns it in its own task
#[tracing::instrument]
pub async fn setup_listener() -> Result<(), Box<dyn Error>> {
    let bind_address = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| {
        warn!("Env BIND_ADDRESS not set, defaulting to 'localhost:25565'");
        String::from("localhost:25565")
    });
    let listener = TcpListener::bind(bind_address).await?;

    tokio::spawn(async move {
        loop {
            let (_socket, _addresss) = match listener.accept().await {
                Ok((socket, address)) => (socket, address),
                Err(err) => {
                    error!("{:?}", err.to_string());
                    continue;
                }
            };

            // TODO: setup connection, add it to global hashmap
        }
    });
    Ok(())
}
