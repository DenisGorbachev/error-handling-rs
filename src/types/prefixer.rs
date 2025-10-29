use std::io::Write;

#[derive(Debug)]
pub struct Prefixer<'w, Writer: Write> {
    pub first_line_prefix: String,
    pub next_line_prefix: String,
    pub writer: &'w mut Writer,
    pub is_first_line: bool,
}

impl<'w, Writer: Write> Prefixer<'w, Writer> {
    pub fn new(first_line_prefix: impl Into<String>, next_line_prefix: impl Into<String>, writer: &'w mut Writer) -> Self {
        Self {
            first_line_prefix: first_line_prefix.into(),
            next_line_prefix: next_line_prefix.into(),
            writer,
            is_first_line: true,
        }
    }
}

impl<'w, Writer: Write> Write for Prefixer<'w, Writer> {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        if self.is_first_line {
            self.writer.write(self.first_line_prefix.as_bytes())?;
        }
        // TODO: prefix each next line with `self.next_line_prefix`
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
