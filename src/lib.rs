pub mod raw;
pub mod writer;

pub use encoding_rs;

use encoding_rs::{Encoding, UTF_8};

/// # Transcoder Builder
///
/// A builder struct to create various `Transcoder`
pub struct TranscoderBuilder {
    from_encoding: &'static Encoding,
    to_encoding: &'static Encoding,
}

impl TranscoderBuilder {
    /// Creates a new [`TranscoderBuilder`].
    pub fn new() -> Self {
        TranscoderBuilder {
            from_encoding: UTF_8,
            to_encoding: UTF_8,
        }
    }
    /// Set the input encoding
    pub fn from_encoding(mut self, encoding: &'static Encoding) -> Self {
        self.from_encoding = encoding;
        self
    }
    /// Set the output encoding
    pub fn to_encoding(mut self, encoding: &'static Encoding) -> Self {
        self.to_encoding = encoding;
        self
    }
}

impl Default for TranscoderBuilder {
    fn default() -> Self {
        Self::new()
    }
}
