# encode_rs_transcode

[![Crates.io](https://img.shields.io/crates/v/encoding_rs_transcode)](https://crates.io/crates/encoding_rs_transcode)
[![docs.rs](https://docs.rs/encoding_rs_transcode/badge.svg)](https://docs.rs/encoding_rs_transcode/)
[![dependency status](https://deps.rs/repo/github/vincentfoulon80/encoding_rs_transcode/status.svg)](https://deps.rs/repo/github/vincentfoulon80/encoding_rs_transcode)
[![Crates.io](https://img.shields.io/crates/l/encoding_rs_transcode)](https://github.com/VincentFoulon80/encoding_rs_transcode/blob/master/LICENSE)

This library allows you to easily transcode text within writers.

The transcoding is performed by [encoding_rs](https://crates.io/crates/encoding_rs), this library just provides a simple builder to ease the use with external writers.

# Use cases

- Writing files in a particular format (e.g. writing a CSV file in WINDOWS_1252, using the [csv](https://crates.io/crates/csv) crate)

# Example

```rust
extern crate csv;
use std::fs::File;
use encoding_rs_transcode::{encoding_rs::WINDOWS_1252, TranscoderBuilder};

fn main() {
    // Create a file writer
    let file = File::create("test.csv").unwrap();

    // Create a transcoder that'll transcode the input to WINDOWS_1252
    let transcoder_writer = TranscoderBuilder::new()
        .to_encoding(WINDOWS_1252)
        .build_writer(file);

    // Create a CSV writer
    let mut csv_writer = csv::Writer::from_writer(transcoder_writer);
    // Write to the CSV file
    csv_writer.write_record(["foo", "bar"]).unwrap();
    csv_writer.write_record(["aceio", "àcéîö"]).unwrap();

    // The CSV file will now be encoded in WINDOWS_1252, without the CSV crate ever
    // aknowledging the final encoding.
    // This can be applied to any writer implementing the `Write` trait.
}
```