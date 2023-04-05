//! Raw transcode from a byte array to another byte array
use encoding_rs::Encoding;

use crate::TranscoderBuilder;

/// # Transcoder Raw
///
/// Raw transcode from a byte array to another byte array.
pub struct TranscoderRaw {
    decoder: &'static Encoding,
    encoder: &'static Encoding,
}

impl TranscoderBuilder {
    /// Build a `TranscoderRaw`
    pub fn build_raw(self) -> TranscoderRaw {
        TranscoderRaw {
            decoder: self.from_encoding,
            encoder: self.to_encoding,
        }
    }
}

impl TranscoderRaw {
    /// Transcode text from one encoding to another
    pub fn transcode(&self, input: &[u8]) -> Vec<u8> {
        let (input, _, _) = self.decoder.decode(input);
        let (output, _, _) = self.encoder.encode(&input);
        output.to_vec()
    }
}
