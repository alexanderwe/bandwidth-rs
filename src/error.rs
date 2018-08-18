#[derive(Debug, Fail)]
pub enum ConfigError {
    #[fail(display = "Invalid config.toml")]
    InvalidConfigFile,
}

#[derive(Debug, Fail)]
pub enum ServiceError {
    #[fail(display = "Failed to listen on interface")]
    InterfaceError,
    #[fail(display = "File {} is missing", _0)]
    MissingFileError(String),
    #[fail(display = "Could not create stats file {}", _0)]
    FileCreationError(String),
    #[fail(display = "Could not write to file {}", _0)]
    FileWriteError(String),
}
