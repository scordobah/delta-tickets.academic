use std::{
    env::var as from_env,
    net::{Ipv4Addr, SocketAddr},
    num::ParseIntError,
    sync::Arc,
};

use lazy_static::lazy_static;

use crate::models::errors::{ErrorKind, Exception};

#[derive(Debug)]
pub struct ServerConfig {
    host: Ipv4Addr,
    port: u16,
}

impl ServerConfig {
    pub fn get_socket_address(&self) -> SocketAddr {
        SocketAddr::new(self.host.into(), self.port)
    }
}

#[derive(Debug)]
pub struct EventsManagerConfig {
    host: String,
    port: u16,
}

impl EventsManagerConfig {
    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_conn_str(&self) -> String {
        format!("http://{}:{}", self.get_host(), self.get_port())
    }
}

#[derive(Debug)]
pub struct AuthServiceConfig {
    host: String,
    port: u16,
}

impl AuthServiceConfig {
    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_conn_str(&self) -> String {
        format!("http://{}:{}", self.get_host(), self.get_port())
    }
}

#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
    events_manager: EventsManagerConfig,
    auth_service: AuthServiceConfig,
}

impl Config {
    pub fn get_server_config(&self) -> &ServerConfig {
        &self.server
    }

    pub fn get_events_manager_config(&self) -> &EventsManagerConfig {
        &self.events_manager
    }

    pub fn get_auth_service_config(&self) -> &AuthServiceConfig {
        &self.auth_service
    }
}

pub trait BuildFromEnvironment {
    fn from_env() -> Result<Self, Exception>
    where
        Self: Sized;
}

impl BuildFromEnvironment for ServerConfig {
    fn from_env() -> Result<Self, Exception> {
        let host: Ipv4Addr = Ipv4Addr::UNSPECIFIED;

        let port: u16 = 80;

        Ok(ServerConfig { host, port })
    }
}

impl BuildFromEnvironment for EventsManagerConfig {
    fn from_env() -> Result<Self, Exception> {
        let host: String = from_env("EVENTS_MANAGER_HOST").map_err(|e| {
            Exception::new(ErrorKind::EnvVar, &format!("EVENTS_MANAGER_HOST- {e}",))
        })?;

        let port: u16 = from_env("EVENTS_MANAGER_PORT")
            .map_err(|e| Exception::new(ErrorKind::EnvVar, &format!("EVENTS_MANAGER_PORT- {e}",)))?
            .parse::<u16>()
            .map_err(|e: ParseIntError| {
                Exception::new(ErrorKind::Parsing, &format!("EVENTS_MANAGER_PORT- {e}",))
            })?;

        Ok(EventsManagerConfig { host, port })
    }
}

impl BuildFromEnvironment for AuthServiceConfig {
    fn from_env() -> Result<Self, Exception> {
        let host: String = from_env("AUTH_SERVICE_HOST")
            .map_err(|e| Exception::new(ErrorKind::EnvVar, &format!("AUTH_SERVICE_HOST- {e}",)))?;

        let port: u16 = from_env("AUTH_SERVICE_PORT")
            .map_err(|e| Exception::new(ErrorKind::EnvVar, &format!("AUTH_SERVICE_PORT- {e}",)))?
            .parse::<u16>()
            .map_err(|e: ParseIntError| {
                Exception::new(ErrorKind::Parsing, &format!("AUTH_SERVICE_PORT- {e}",))
            })?;

        Ok(AuthServiceConfig { host, port })
    }
}

impl BuildFromEnvironment for Config {
    fn from_env() -> Result<Self, Exception> {
        let server: ServerConfig = ServerConfig::from_env()?;

        let events_manager: EventsManagerConfig = EventsManagerConfig::from_env()?;

        let auth_service: AuthServiceConfig = AuthServiceConfig::from_env()?;

        Ok(Config {
            server,
            events_manager,
            auth_service,
        })
    }
}

lazy_static! {
    pub static ref CONFIG: Arc<Config> = Arc::new(Config::from_env().unwrap_or_else(|e| {
        panic!("Failed to load microservice configuration: {e}");
    }));
}
