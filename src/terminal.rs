use core::fmt;
pub use core::fmt::Result;
pub use core::fmt::Write;
use core::ops::{Deref, DerefMut};
use spin::Mutex;
use volatile::Volatile;

// making allocation of Terminal when first used to allow for pointer deref
lazy_static! {
    pub static ref TERMINAL: Mutex<Terminal> = Mutex::new(Terminal::new());
}

// Prints formatted string
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::terminal::_print(format_args!($($arg)*)));
}

// Prints formatted string with newline
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    TERMINAL.lock().writer.write_fmt(args).unwrap();
}

/// An object representative of the whole terminal
pub struct Terminal {
    pub writer: Writer,
}

/// The Terminal full impl
impl Terminal {
    /// Simply creates a new Terminal
    pub fn new() -> Terminal {
        Terminal {
            writer: Writer {
                column_position: 0,
                color_code: ColorCode::new(Color::Yellow, Color::Black),
                buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
            },
        }
    }
}

/// The main writing object that handles writing text
#[repr(packed)]
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

/// making standard macros work
impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_string(s);
        Ok(())
    }
}

/// Writer full impl
impl Writer {
    /// write a string to the buffer
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// write individual char or byte to the buffer
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.newline();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col].write(*ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });

                self.column_position += 1;
            }
        }
    }

    /// manage newlines
    fn newline(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// managing row clearing
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

/// Representative of each char on the screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(packed)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

/// For the volatile package (standard Deref)
impl Deref for ScreenChar {
    type Target = ScreenChar;

    fn deref(&self) -> &Self::Target {
        return &self;
    }
}

/// For the volatile package (standard DerefMut)
impl DerefMut for ScreenChar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return self;
    }
}

/// The height of the Screen Buffer (text)
const BUFFER_HEIGHT: usize = 25;
/// The width of the Screen Buffer (text)
const BUFFER_WIDTH: usize = 80;

/// The underlying type for the Screen Buffer (text)
#[repr(packed)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// The type for holding colours
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(packed)]
pub struct ColorCode(u8);

/// The colour type implementation
impl ColorCode {
    /// Generates ColorCode bits based on foreground and background colour
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// The colours of text
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}
