use std::fmt;
use std::io::{self, Write};

/// A [`Write`] adapter that prefixes each written line.
///
/// This type uses a `dyn Write` instead of `impl Write` to avoid a trait-recursion explosion in
/// [`crate::writeln_error_to_writer`].
pub struct Prefixer<'w> {
    /// Prefix for the very first line.
    pub first_line_prefix: String,
    /// Prefix for subsequent lines.
    pub next_line_prefix: String,
    /// The underlying writer.
    pub writer: &'w mut dyn Write,
    /// Whether the next write is still on the first line.
    pub is_first_line: bool,
    /// Whether the next write should include a prefix.
    pub needs_prefix: bool,
}

impl<'w> fmt::Debug for Prefixer<'w> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Prefixer")
            .field("first_line_prefix", &self.first_line_prefix)
            .field("next_line_prefix", &self.next_line_prefix)
            .field("is_first_line", &self.is_first_line)
            .field("needs_prefix", &self.needs_prefix)
            .finish()
    }
}

impl<'w> Prefixer<'w> {
    /// Creates a new prefixing writer with the provided line prefixes.
    pub fn new(first_line_prefix: impl Into<String>, next_line_prefix: impl Into<String>, writer: &'w mut dyn Write) -> Self {
        Self {
            first_line_prefix: first_line_prefix.into(),
            next_line_prefix: next_line_prefix.into(),
            writer,
            is_first_line: true,
            needs_prefix: true,
        }
    }
}

impl<'w> Write for Prefixer<'w> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let mut start = 0;
        while start < buf.len() {
            if self.needs_prefix {
                let prefix = if self.is_first_line { &self.first_line_prefix } else { &self.next_line_prefix };
                self.writer.write_all(prefix.as_bytes())?;
                self.is_first_line = false;
                self.needs_prefix = false;
            }

            match buf[start..].iter().position(|&b| b == b'\n') {
                Some(relative_idx) => {
                    let end = start + relative_idx + 1;
                    self.writer.write_all(&buf[start..end])?;
                    start = end;
                    self.needs_prefix = true;
                }
                None => {
                    self.writer.write_all(&buf[start..])?;
                    start = buf.len();
                }
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}
