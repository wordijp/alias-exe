use std::io::{self, Write};
use std::sync::Mutex;

use termcolor::{Color, ColorChoice, ColorSpec, WriteColor, BufferWriter};
use atty::Stream;

use crate::lib::encode;

pub fn keywrite(text: &str) -> io::Result<String> {
    if !atty::is(Stream::Stdout) {
        return Ok(encode::to_utf8_string(text.as_bytes()));
    }

    lazy_static! {
        static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(BufferWriter::stdout(ColorChoice::Auto).buffer()));
    }

    let mut w = WRITER.lock().unwrap();
    w.set_fg_color(Color::Green)?;

    w.write(text)
}

pub fn ewrite(text: &str) -> io::Result<String> {
    if !atty::is(Stream::Stderr) {
        return Ok(encode::to_utf8_string(text.as_bytes()));
    }

    lazy_static! {
        static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(BufferWriter::stderr(ColorChoice::Auto).buffer()));
    }

    let mut w = WRITER.lock().unwrap();
    w.set_fg_color(Color::Rgb(0xFF, 0, 0))
        .or_else(|_| w.set_fg_color(Color::Red))?;

    w.write(text)
}

// ---

struct Writer {
    buf: termcolor::Buffer,
    spec: termcolor::ColorSpec,
}

impl Writer {
    fn new(buf: termcolor::Buffer) -> Writer {
        Writer {
            buf,
            spec: ColorSpec::new(),
        }
    }

    fn set_fg_color(&mut self, fg: Color) -> io::Result<()> {
        self.spec.set_fg(Some(fg));
        self.buf.set_color(&self.spec)
    }

    fn write(&mut self, text: &str) -> io::Result<String> {
        write!(&mut self.buf, "{}", text)?;
        self.buf.reset()?;
        let s = encode::to_utf8_string(&self.buf.as_slice());
        self.buf.clear();
        Ok(s)
    }
}
