use thiserror::Error;

#[derive(Debug, Error)]
pub enum PngError {
    #[error("invalid chunk type")]
    ChunkTypeError,
    #[error("invalid chunk data")]
    ChunkError,
    #[error("chunk data format utf8 error")]
    FormatError(#[from] std::string::FromUtf8Error),
    #[error("chunk data remove error from png")]
    ChunkRemoveError,
    #[error("create png error")]
    CreatePngError,
    #[error("IO error")]
    IOError(#[from] std::io::Error),
}
