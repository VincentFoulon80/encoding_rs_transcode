//! A basic writer that transcodes its input into another writer

use crate::TranscoderBuilder;
use encoding_rs::Encoding;
use std::io::{Seek, Write};

/// # Transcoder writer
///
/// A basic writer that transcodes its input into another writer
pub struct TranscoderWriter<W: Write> {
    decoder: &'static Encoding,
    encoder: &'static Encoding,
    inner: W,
}

impl TranscoderBuilder {
    /// Build a `TranscoderWriter`
    pub fn build_writer<W: Write>(self, writer: W) -> TranscoderWriter<W> {
        TranscoderWriter {
            decoder: self.from_encoding,
            encoder: self.to_encoding,
            inner: writer,
        }
    }
}

impl<W> TranscoderWriter<W>
where
    W: Write,
{
    /// Get the inner Writer, dropping the `TranscoderWriter` instance
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W> Write for TranscoderWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let org_len = buf.len();
        let (buf, _, _) = self.decoder.decode(buf);
        let (buf, _, _) = self.encoder.encode(&buf);
        self.inner.write(&buf).and(Ok(org_len))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl<W> Seek for TranscoderWriter<W>
where
    W: Write + Seek,
{
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.inner.seek(pos)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use encoding_rs::WINDOWS_1252;

    use crate::TranscoderBuilder;

    /// Basic writer that stores its write calls in memory
    /// Only use for testing purpose
    #[derive(Default)]
    pub struct MemWriter {
        pub mem: Vec<u8>,
    }

    impl Write for MemWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.mem.append(&mut buf.to_vec());
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn utf8_to_win1252() {
        let expectation: [u8; 6] = [233, 224, 231, 234, 226, 238];
        let input = "éàçêâî";

        let mem_writer = MemWriter::default();

        let mut writer = TranscoderBuilder::new()
            .to_encoding(WINDOWS_1252)
            .build_writer(mem_writer);

        writer.write_all(input.as_bytes()).unwrap();

        assert!(writer.into_inner().mem == expectation);
    }

    #[test]
    fn win1252_to_utf8() {
        let input: [u8; 6] = [233, 224, 231, 234, 226, 238];
        let expectation = "éàçêâî";

        let mem_writer = MemWriter::default();

        let mut writer = TranscoderBuilder::new()
            .from_encoding(WINDOWS_1252)
            .build_writer(mem_writer);

        writer.write_all(&input).unwrap();

        assert!(String::from_utf8_lossy(&writer.into_inner().mem) == expectation);
    }
}
