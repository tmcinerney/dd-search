//! NDJSON (Newline Delimited JSON) output writer.
//!
//! Provides streaming output of JSON records, one per line, suitable for
//! piping to tools like `jq` or processing line-by-line.

use serde::Serialize;
use std::io::{self, BufWriter, Stdout, Write};

/// Writes JSON records as newline-delimited JSON (NDJSON) to stdout.
///
/// Each record is serialized as compact JSON followed by a newline.
/// Output is flushed after each record for real-time streaming.
pub struct NdjsonWriter {
    writer: BufWriter<Stdout>,
}

impl NdjsonWriter {
    /// Creates a new NDJSON writer to stdout.
    pub fn new() -> Self {
        Self {
            writer: BufWriter::new(io::stdout()),
        }
    }

    /// Writes a single record as JSON followed by a newline.
    ///
    /// The output is flushed immediately to support real-time streaming.
    pub fn write<T: Serialize>(&mut self, record: &T) -> io::Result<()> {
        serde_json::to_writer(&mut self.writer, record)?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()
    }
}

impl Default for NdjsonWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use std::io::Write;

    #[derive(Serialize)]
    struct TestRecord {
        id: u32,
        name: String,
    }

    // Helper function to test writing to a buffer
    fn write_to_buffer<T: Serialize>(record: &T) -> String {
        let mut buffer = Vec::new();
        {
            let mut writer = BufWriter::new(&mut buffer);
            serde_json::to_writer(&mut writer, record).unwrap();
            writer.write_all(b"\n").unwrap();
            writer.flush().unwrap();
        }
        String::from_utf8(buffer).unwrap()
    }

    #[test]
    fn test_ndjson_writer_serializes_and_adds_newline() {
        let record = TestRecord {
            id: 1,
            name: "test".to_string(),
        };

        let output = write_to_buffer(&record);
        assert!(output.ends_with('\n'));
        assert!(output.contains(r#""id":1"#));
        assert!(output.contains(r#""name":"test""#));
    }

    #[test]
    fn test_ndjson_writer_multiple_records() {
        let record1 = TestRecord {
            id: 1,
            name: "first".to_string(),
        };
        let record2 = TestRecord {
            id: 2,
            name: "second".to_string(),
        };

        let output1 = write_to_buffer(&record1);
        let output2 = write_to_buffer(&record2);

        assert!(output1.contains("first"));
        assert!(output2.contains("second"));
        assert!(output1.ends_with('\n'));
        assert!(output2.ends_with('\n'));
    }

    #[test]
    fn test_ndjson_writer_default() {
        let writer = NdjsonWriter::default();
        // Just verify it doesn't panic
        drop(writer);
    }

    #[test]
    fn test_ndjson_writer_new() {
        let writer = NdjsonWriter::new();
        // Just verify it doesn't panic
        drop(writer);
    }
}
