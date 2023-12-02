/// Error types for reading system dictionary
#[derive(Debug)]
pub enum ReadError {
    /// Decoding error
    Decode(ruzstd::frame_decoder::FrameDecoderError),

    /// Vibrato related error
    Vibrato(vibrato::errors::VibratoError),

    /// IO error
    Io(std::io::Error),
}

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ReadError::Decode(..) => write!(f, "Failed to decode system dictionary"),
            ReadError::Vibrato(..) => write!(f, "Failed to read system dictionary to vibrato"),
            ReadError::Io(..) => write!(f, "Failed to read system dictionary"),
        }
    }
}

impl std::error::Error for ReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ReadError::Decode(ref e) => Some(e),
            ReadError::Vibrato(ref e) => Some(e),
            ReadError::Io(ref e) => Some(e),
        }
    }
}

impl From<ruzstd::frame_decoder::FrameDecoderError> for ReadError {
    fn from(e: ruzstd::frame_decoder::FrameDecoderError) -> ReadError {
        ReadError::Decode(e)
    }
}

impl From<vibrato::errors::VibratoError> for ReadError {
    fn from(e: vibrato::errors::VibratoError) -> ReadError {
        ReadError::Vibrato(e)
    }
}

impl From<std::io::Error> for ReadError {
    fn from(e: std::io::Error) -> ReadError {
        ReadError::Io(e)
    }
}
