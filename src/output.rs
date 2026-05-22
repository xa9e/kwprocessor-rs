use std::fs::OpenOptions;
use std::io::{self, Write};

use crate::config::Config;
use crate::constants::OUTPUT_BUFFER_CAPACITY;
use crate::error::{KwpError, Result};

pub(crate) struct Output {
    writer: Box<dyn Write>,
    buf: Vec<u8>,
}

impl Output {
    pub(crate) fn new(writer: Box<dyn Write>) -> Self {
        Self {
            writer,
            buf: Vec::with_capacity(OUTPUT_BUFFER_CAPACITY),
        }
    }

    #[inline]
    pub(crate) fn push_line(&mut self, line: &[u8]) -> io::Result<()> {
        let need = line.len() + 1;

        if self.buf.len() + need > OUTPUT_BUFFER_CAPACITY {
            self.flush()?;
        }

        if need > OUTPUT_BUFFER_CAPACITY {
            self.writer.write_all(line)?;
            self.writer.write_all(b"\n")?;
            return Ok(());
        }

        self.buf.extend_from_slice(line);
        self.buf.push(b'\n');

        Ok(())
    }

    pub(crate) fn flush(&mut self) -> io::Result<()> {
        if !self.buf.is_empty() {
            self.writer.write_all(&self.buf)?;
            self.buf.clear();
        }

        self.writer.flush()
    }
}

pub(crate) fn open_output(config: &Config) -> Result<Box<dyn Write>> {
    if let Some(path) = &config.output_file {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|err| KwpError::Message(format!("ERROR: {}: {err}", path.display())))?;
        Ok(Box::new(file))
    } else {
        Ok(Box::new(io::stdout()))
    }
}
