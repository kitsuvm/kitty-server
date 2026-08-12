//! The connection configuration module.

use std::{
    fs,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

/// The SSH connection configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SSHServer {
    /// The name of the SSH connection.
    pub name: Option<String>,
    /// The host of the SSH connection.
    pub host: String,
    /// The port of the SSH connection.
    pub port: Option<u16>,
    /// The username of the SSH connection.
    pub username: Option<String>,
}

/// The servers in the connection configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Servers {
    /// The SSH servers in the connection configuration.
    pub ssh_servers: Vec<SSHServer>,
}

impl Servers {
    /// Get the file path of the servers configuration file.
    pub fn file_path(project_dirs: &ProjectDirs) -> PathBuf {
        project_dirs.config_dir().join("servers.toml")
    }

    /// Load the servers configuration from a file. If the file does not exist, create a new one with default values.
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, String> {
        if !fs::exists(&path).map_err(|e| {
            tracing::error!(
                "Could not check if connection configuration file exists: {}",
                e
            );
            e.to_string()
        })? {
            tracing::warn!("Connection configuration file does not exist, creating a new one.");
            let default_servers = Self {
                ssh_servers: Vec::new(),
            };

            fs::create_dir_all(path.as_ref().parent().ok_or_else(|| {
                tracing::error!("Could not get parent directory of connection configuration file.");
                "Could not get parent directory of connection configuration file.".to_string()
            })?)
            .map_err(|e| {
                tracing::error!("Could not create connection configuration directory: {}", e);
                e.to_string()
            })?;

            default_servers.save_to_file(&path).map_err(|e| {
                tracing::error!("Could not create connection configuration file: {}", e);
                e.to_string()
            })?;
        }

        let data = fs::read(path).map_err(|e| {
            tracing::error!("Could not read connection configuration file: {}", e);
            e.to_string()
        })?;

        toml::from_slice::<Self>(&data).map_err(|e| {
            tracing::error!("Could not parse connection configuration file: {}", e);
            e.to_string()
        })
    }

    /// Save the servers configuration to a file.
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let data = toml::to_string(self).map_err(|e| {
            tracing::error!("Could not serialize connection configuration: {}", e);
            e.to_string()
        })?;

        fs::write(path, data).map_err(|e| {
            tracing::error!("Could not write connection configuration file: {}", e);
            e.to_string()
        })
    }
}

/// The servers manager, which manages the servers in the connection configuration.
#[derive(Debug, Clone)]
pub struct ServersManager {
    /// The servers in the connection configuration.
    pub servers: Servers,
}

impl ServersManager {
    /// Check if the servers manager is empty.
    pub fn is_empty(&self) -> bool {
        self.servers.ssh_servers.is_empty()
    }
}

impl From<Servers> for ServersManager {
    fn from(servers: Servers) -> Self {
        Self { servers }
    }
}

/// The state of the servers manager.
#[derive(Debug, Clone, Default)]
pub enum ServersState {
    /// The servers manager is loading.
    #[default]
    Loading,
    /// The servers manager has loaded successfully.
    Data(ServersManager),
    /// The servers manager has failed to load.
    Error(String),
}

impl From<Result<Servers, String>> for ServersState {
    fn from(result: Result<Servers, String>) -> Self {
        match result {
            Ok(servers) => Self::Data(servers.into()),
            Err(err) => Self::Error(err),
        }
    }
}
