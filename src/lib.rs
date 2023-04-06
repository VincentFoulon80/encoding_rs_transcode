//! # encode_rs_transcode
//!
//! This library allows you to easily transcode text within writers.
//!
//! The transcoding is performed by [encoding_rs](https://crates.io/crates/encoding_rs), this library just provides a simple builder to ease the use with external writers.
//!
//! # Use cases
//!
//! - Writing files in a particular format (e.g. writing a CSV file in WINDOWS_1252, using the [csv](https://crates.io/crates/csv) crate)
//!
//! # Example
//!
//! ```rust,ignore
//! extern crate csv;
//! use std::fs::File;
//! use encoding_rs_transcode::{encoding_rs::WINDOWS_1252, TranscoderBuilder};
//!
//! fn main() {
//!     // Create a file writer
//!     let file = File::create("test.csv").unwrap();
//!
//!     // Create a transcoder that'll transcode the input to WINDOWS_1252
//!     let transcoder_writer = TranscoderBuilder::new()
//!     //  .from_encoding(UTF_8) // implied by new()
//!         .to_encoding(WINDOWS_1252)
//!         .build_writer(file);
//!
//!     // Create a CSV writer
//!     let mut csv_writer = csv::Writer::from_writer(transcoder_writer);
//!     // Write to the CSV file
//!     csv_writer.write_record(["foo", "bar"]).unwrap();
//!     csv_writer.write_record(["aceio", "àcéîö"]).unwrap();
//!
//!     // The CSV file will now be encoded in WINDOWS_1252, without the CSV crate ever
//!     // aknowledging the final encoding.
//!     // This can be applied to any writer implementing the `Write` trait.
//! }
//! ```
pub mod raw;
pub mod writer;

pub use encoding_rs;

use encoding_rs::{Encoding, UTF_8};

/// A builder struct to create various Transcoders
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
