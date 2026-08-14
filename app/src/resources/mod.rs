use std::{
    borrow::Cow,
    fs,
    io::{Error, ErrorKind, Result},
    path::{Path, PathBuf},
    sync::Arc,
};

use directories::ProjectDirs;
use serde::{Serialize, de::DeserializeOwned};

pub mod app_config;
pub mod hosts;

/// A trait for resources that can be located in the project directories.
pub trait ResourceLocation {
    /// Get the directory where the resource is stored.
    fn dir(project_dirs: &ProjectDirs) -> Cow<'_, Path>;

    /// Get the filename of the resource.
    fn filename() -> &'static str;
}

/// Get the path of a resource in the project directories.
pub fn path<T: ResourceLocation>(project_dirs: &ProjectDirs) -> PathBuf {
    T::dir(project_dirs).join(T::filename())
}

/// A trait for resources that can be managed by the ResourceManager.
pub trait Resource: Sized {
    /// Load the resource from the project directories.
    fn load_from<P: AsRef<Path>>(path: P) -> Result<Self>;

    /// Save the resource to the project directories.
    fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<()>;
}

impl<T> Resource for T
where
    T: Serialize + DeserializeOwned + Default,
{
    fn load_from<P: AsRef<Path>>(path: P) -> Result<Self> {
        if !path.as_ref().exists() {
            tracing::warn!(
                "Resource file {} does not exist, creating a new one.",
                path.as_ref().display()
            );

            let defaults = Self::default();

            defaults.save_to(path)?;

            return Ok(defaults);
        }

        let data = fs::read(&path)?;

        toml::from_slice(&data).map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }

        let data = toml::to_string(self).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        fs::write(path, data)
    }
}

/// A trait for resources that can be managed by the ResourceManager and have a location in the project directories.
pub trait ManagedResource: Resource + ResourceLocation {
    /// Load the resource from the project directories.
    fn load(project_dirs: &ProjectDirs) -> Result<Self> {
        Self::load_from(path::<Self>(project_dirs))
    }

    /// Save the resource to the project directories.
    fn save(&self, project_dirs: &ProjectDirs) -> Result<()> {
        self.save_to(path::<Self>(project_dirs))
    }
}

impl<T> ManagedResource for T where T: Resource + ResourceLocation {}

/// An inner struct for [`ResourceManager`] that holds the resource manager data.
#[derive(Debug)]
struct ResourceManagerInner {
    /// The project directories where resources are stored.
    project_dirs: ProjectDirs,
}

/// A resource manager that can load and save resources.
#[derive(Debug, Clone)]
pub struct ResourceManager {
    /// The inner struct that holds the resource manager data.
    inner: Arc<ResourceManagerInner>,
}

impl ResourceManager {
    /// Create a new [`ResourceManager`] with the given project directories.
    pub fn new(project_dirs: ProjectDirs) -> Self {
        Self {
            inner: Arc::new(ResourceManagerInner { project_dirs }),
        }
    }

    /// Load a resource from a specific path.
    pub fn load_from<T: Resource>(&self, path: impl AsRef<Path>) -> Result<T> {
        T::load_from(path)
    }

    /// Save a resource to a specific path.
    pub fn save_to<T: Resource>(&self, resource: &T, path: impl AsRef<Path>) -> Result<()> {
        resource.save_to(path)
    }

    /// Load a resource.
    pub fn load<T: ManagedResource>(&self) -> Result<T> {
        T::load(&self.inner.project_dirs)
    }

    /// Save a resource.
    pub fn save<T: ManagedResource>(&self, resource: &T) -> Result<()> {
        resource.save(&self.inner.project_dirs)
    }
}
