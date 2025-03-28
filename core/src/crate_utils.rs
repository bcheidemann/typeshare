use cargo_toml::Manifest;
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]

/// A crate name.
pub struct CrateName(String);

impl Display for CrateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for CrateName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for CrateName {
    fn from(value: &str) -> Self {
        CrateName(value.to_string())
    }
}

/// When using single file output we put all types into a single virtual name space.
pub const SINGLE_FILE_CRATE_NAME: CrateName = CrateName(String::new());

#[derive(Debug, thiserror::Error)]
pub enum FindCrateNameError {
    #[error("Unexpected error parsing {cargo_toml_path}: {err}")]
    CargoTomlError {
        cargo_toml_path: PathBuf,
        err: cargo_toml::Error,
    },
    #[error("Could not find a Cargo.toml file for source file at {source_file_path}")]
    MissingCargoToml { source_file_path: PathBuf },
    #[error("Source file at {source_file_path} is not part of a package")]
    SourceFileNotInPackage { source_file_path: PathBuf },
    #[error("Invalid source file path ({source_file_path}): {err}")]
    InvalidSourceFilePath {
        source_file_path: PathBuf,
        err: std::io::Error,
    },
}

impl CrateName {
    /// View this crate name as a string slice.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Extract the crate name from a give path.
    pub fn find_crate_name(path: &Path) -> Result<Self, FindCrateNameError> {
        let manifest = Self::find_cargo_toml(&path)?;
        let crate_name = manifest
            .package
            .ok_or_else(|| FindCrateNameError::SourceFileNotInPackage {
                source_file_path: path.to_path_buf(),
            })?
            .name;
        Ok(CrateName(crate_name))
    }

    fn find_cargo_toml(
        source_file_path: impl AsRef<Path> + Clone,
    ) -> Result<Manifest, FindCrateNameError> {
        let dir = std::path::absolute(&source_file_path).map_err(|err| {
            FindCrateNameError::InvalidSourceFilePath {
                source_file_path: source_file_path.as_ref().to_path_buf(),
                err,
            }
        })?;
        let mut dir = dir.as_path();

        loop {
            dir = dir
                .parent()
                .ok_or_else(|| FindCrateNameError::MissingCargoToml {
                    source_file_path: source_file_path.as_ref().to_path_buf(),
                })?;
            let candidate_cargo_toml = dir.join("Cargo.toml");
            match Manifest::from_path(&candidate_cargo_toml) {
                Ok(manifest) => return Ok(manifest),
                Err(cargo_toml::Error::Io(_)) => continue,
                Err(err) => {
                    return Err(FindCrateNameError::CargoTomlError {
                        cargo_toml_path: candidate_cargo_toml.to_path_buf(),
                        err,
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_crate_name() {
        let path = Path::new("./core/src/language/mod.rs");
        assert_eq!(
            "typeshare-core",
            CrateName::find_crate_name(path).unwrap().as_str()
        );
    }
}
