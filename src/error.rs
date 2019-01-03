extern crate rusqlite;
extern crate config;
extern crate glob;
extern crate serde_json;

#[derive(Debug)]
pub enum Error {
    Rusqlite(rusqlite::Error),
    Config(config::ConfigError),
    PatternError(glob::PatternError),
    GlobError(glob::GlobError),
    Io(std::io::Error),
    SerdeJson(serde_json::Error),
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Error {
        Error::Rusqlite(err)
    }
}

impl From<config::ConfigError> for Error {
    fn from(err: config::ConfigError) -> Error {
        Error::Config(err)
    }
}

impl From<glob::PatternError> for Error {
    fn from(err: glob::PatternError) -> Error {
        Error::PatternError(err)
    }
}

impl From<glob::GlobError> for Error {
    fn from(err: glob::GlobError) -> Error {
        Error::GlobError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::SerdeJson(err)
    }
}
