use std::io::Write;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Prefixer<Writer: Write> {
    pub first_line_prefix: String,
    pub next_line_prefix: String,
    pub writer: Writer,
    pub is_first_line: bool,
}

impl<Writer: Write> Prefixer<Writer> {
    pub fn new(first_line_prefix: impl Into<String>, next_line_prefix: impl Into<String>, writer: Writer) -> Self {
        Self {
            first_line_prefix: first_line_prefix.into(),
            next_line_prefix: next_line_prefix.into(),
            writer,
            is_first_line: true,
        }
    }
}

impl<Writer: Write> Write for Prefixer<Writer> {
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
