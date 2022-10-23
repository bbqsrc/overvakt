use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("internal error")]
    InternalError,
    #[error("io error: {error}")]
    IoError {
        #[from]
        #[source]
        error: ::std::io::Error,
    },
}
