mod config;
mod helpers;

use config::{AppConfig, LogLevel};
use helpers::generate_unique_id;
use shared::network::NetworkConfig;

fn main() {
    let app_config = AppConfig {
        debug_mode: true,
        log_level: LogLevel::Debug,
    };

    let network_config = NetworkConfig::new("localhost", 8080);
    let unique_id = generate_unique_id();

    println!("App Configuration: {:?}", app_config);
    println!(
        "Network Config: {}:{}",
        network_config.host, network_config.port
    );
    println!("Unique Session ID: {}", unique_id);
}
