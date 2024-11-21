#[derive(Debug)]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
}

impl NetworkConfig {
    pub fn new(host: &str, port: u16) -> Self {
        NetworkConfig {
            host: host.to_string(),
            port,
        }
    }

    pub fn connection_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
