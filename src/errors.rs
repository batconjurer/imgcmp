use thiserror::Error;
use image::ImageError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected io error in reading files: {}", .0)]
    IO(#[from] std::io::Error),
    #[error("Error encountered in image: {}", .0)]
    Image(#[from] ImageError),
}