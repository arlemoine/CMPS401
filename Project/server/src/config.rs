// src/config.rs
pub struct Config {
    pub host: &'static str,
    pub port: u16,
    pub log_level: tracing::Level,
}

impl Config {
    pub fn default() -> Self {
        Self {
            host: "127.0.0.1",
            port: 3001,
            log_level: tracing::Level::INFO,
        }
    }

    pub fn socket_addr(&self) -> std::net::SocketAddr {
        std::net::SocketAddr::from(([127,0,0,1], self.port))
    }
}
