//! The connection configuration module.

use std::{cmp::Ordering, io, path::Path};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::resources::{ResourceLocation, ResourceManager};

/// A trait for hosts that can be managed by the [`HostsManager`].
pub trait Host {
    /// Get the title of the host for display purposes.
    fn title(&self) -> String;

    /// Get the subtitle of the host for display purposes.
    fn subtitle(&self) -> Option<String>;

    /// Get the hash of the host for identification purposes.
    fn hash(&self) -> String;
}

/// SSH Host information for connection.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SshHost {
    /// The name of the profile for display purposes.
    pub name: Option<String>,
    /// The host/IP address of the SSH server.
    pub host: String,
    /// The port of the SSH server. If not specified, the default port 22 will be used.
    pub port: Option<u16>,
    /// The username for the SSH connection. If not specified, the current system username will be used.
    pub username: Option<String>,
}

impl Host for SshHost {
    fn title(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| match (&self.username, &self.port) {
                (Some(username), Some(port)) => format!("{}@{}:{}", username, self.host, port),
                (Some(username), None) => format!("{}@{}", username, self.host),
                (None, Some(port)) => format!("{}:{}", self.host, port),
                (None, None) => self.host.clone(),
            })
    }

    fn subtitle(&self) -> Option<String> {
        self.name
            .as_ref()
            .map(|_| match (&self.username, &self.port) {
                (Some(username), Some(port)) => format!("{}@{}:{}", username, self.host, port),
                (Some(username), None) => format!("{}@{}", username, self.host),
                (None, Some(port)) => format!("{}:{}", self.host, port),
                (None, None) => self.host.clone(),
            })
    }

    fn hash(&self) -> String {
        hex::encode(Sha256::digest(match (&self.username, &self.port) {
            (Some(username), Some(port)) => {
                format!("app+ssh://{}@{}:{}", username, self.host, port)
            }
            (Some(username), None) => format!("app+ssh://{}@{}", username, self.host),
            (None, Some(port)) => format!("app+ssh://{}:{}", self.host, port),
            (None, None) => format!("app+ssh://{}", self.host),
        }))
    }
}

/// Host information for connection.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AppHost {
    /// SSH Host information for connection.
    Ssh(SshHost),
}

impl<T: Into<SshHost>> From<T> for AppHost {
    fn from(host: T) -> Self {
        Self::Ssh(host.into())
    }
}

impl Host for AppHost {
    fn title(&self) -> String {
        match self {
            AppHost::Ssh(ssh_host) => ssh_host.title(),
        }
    }

    fn subtitle(&self) -> Option<String> {
        match self {
            AppHost::Ssh(ssh_host) => ssh_host.subtitle(),
        }
    }

    fn hash(&self) -> String {
        match self {
            AppHost::Ssh(ssh_host) => ssh_host.hash(),
        }
    }
}

/// A collection of hosts from the application files.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppHosts {
    /// The hosts from the application files.
    pub hosts: Vec<AppHost>,
}

impl ResourceLocation for AppHosts {
    fn dir(project_dirs: &ProjectDirs) -> std::borrow::Cow<'_, Path> {
        project_dirs.config_dir().into()
    }

    fn filename() -> &'static str {
        "hosts.toml"
    }
}

/// A practical host that can be used for connection.
#[derive(Debug, Clone)]
pub enum PraticalHost {
    /// A host managed by the application files.
    App(AppHost),
}

impl<T: Into<AppHost>> From<T> for PraticalHost {
    fn from(host: T) -> Self {
        Self::App(host.into())
    }
}

impl Host for PraticalHost {
    fn title(&self) -> String {
        match self {
            PraticalHost::App(app_host) => app_host.title(),
        }
    }

    fn subtitle(&self) -> Option<String> {
        match self {
            PraticalHost::App(app_host) => app_host.subtitle(),
        }
    }

    fn hash(&self) -> String {
        match self {
            PraticalHost::App(app_host) => app_host.hash(),
        }
    }
}

impl PartialEq for PraticalHost {
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}

impl Eq for PraticalHost {}

impl PartialOrd for PraticalHost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.title().cmp(&other.title()))
    }
}

impl Ord for PraticalHost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.title().cmp(&other.title())
    }
}

/// Build the practical hosts from the application files.
pub fn build_pratical_hosts(app_hosts: &AppHosts) -> Vec<PraticalHost> {
    let mut pratical_hosts: Vec<PraticalHost> = app_hosts
        .hosts
        .iter()
        .map(|host| PraticalHost::App(host.clone()))
        .collect();

    pratical_hosts.sort();
    pratical_hosts
}

/// The manager for the hosts from the application files.
#[derive(Debug, Clone)]
pub struct HostsManager {
    /// The hosts from the application files.
    app_hosts: AppHosts,
    /// The practical hosts that can be used for connection.
    pratical_hosts: Vec<PraticalHost>,
}

impl HostsManager {
    /// Create a new [`HostsManager`] from the given [`ResourceManager`].
    pub fn new(resource_manager: &ResourceManager) -> Self {
        let app_hosts = resource_manager
            .load::<AppHosts>()
            .inspect_err(|e| tracing::warn!("Can't load hosts: {}", e))
            .unwrap_or_default();

        Self {
            pratical_hosts: build_pratical_hosts(&app_hosts),
            app_hosts,
        }
    }

    /// Get the practical hosts that can be used for connection.
    pub fn get(&self) -> &Vec<PraticalHost> {
        &self.pratical_hosts
    }

    /// Check if has any hosts to connect to.
    pub fn is_empty(&self) -> bool {
        self.pratical_hosts.is_empty()
    }

    /// Add a new host to the application files and save it using the given [`ResourceManager`].
    pub fn push(
        &mut self,
        resource_manager: &ResourceManager,
        host: AppHost,
    ) -> Result<(), io::Error> {
        self.app_hosts.hosts.push(host);
        self.pratical_hosts = build_pratical_hosts(&self.app_hosts);
        resource_manager.save(&self.app_hosts)?;
        Ok(())
    }
}
