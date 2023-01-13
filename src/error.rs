use thiserror::Error;



#[derive(Debug, Error)]
pub enum PngError {
  #[error("invalid chunk type")]
    ChunkTypeError
}
