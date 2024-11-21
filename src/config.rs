#[derive(Debug)]
pub struct AppConfig {
    pub debug_mode: bool,
    pub log_level: LogLevel,
}

#[derive(Debug)]
pub enum LogLevel {
    Info,
    Debug,
    Error,
}
