use shared::network::NetworkConfig;
use uuid::Uuid;

fn main() {
    let network_config = NetworkConfig::new("127.0.0.1", 9090);
    let unique_id = Uuid::new_v4().to_string();

    println!("CLI Application");
    println!("Network Config: {}", network_config.connection_string());
    println!("Session ID: {}", unique_id);
}
