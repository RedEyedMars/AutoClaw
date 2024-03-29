
use image;
use std::io;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O error")]
    Io(#[cause] io::Error),
    #[fail(display = "Failed to read CString from file that contains 0")]
    FileContainsNil,
    #[fail(display = "Failed get executable path")]
    FailedToGetExePath,
    #[fail(display = "Failed to load image {}", name)]
    FailedToLoadImage {
        name: String,
        #[cause]
        inner: image::ImageError,
    },
    #[fail(display = "Image {} is not RGBA", name)]
    ImageIsNotRgba { name: String },
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}
