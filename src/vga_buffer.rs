use core::fmt::{Arguments, Result, Write};
use spin::Mutex;
use lazy_static::lazy_static;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

lazy_static! {
    static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: BUFFER_HEIGHT,
        color_code: ColorCode::new(0xf, 0x0),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[derive(Copy, Clone)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: u8, background: u8) -> ColorCode {
        ColorCode(background << 4 | foreground)
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
struct ScreenChar {
    ascii_caracter: u8,
    color_code: ColorCode
}

struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer
}

impl Writer {
    #[inline]
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line()
                }

                let row: usize = BUFFER_HEIGHT - self.row_position;
                let column: usize = self.column_position;

                let color_code: ColorCode = self.color_code;
                self.buffer.chars[row][column] = ScreenChar {
                    ascii_caracter: byte,
                    color_code
                };
                self.column_position += 1;
            }
        }
    }

    #[inline]
    fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                0x20..0xfe | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }

    #[inline]
    fn new_line(&mut self) {
        if self.row_position != 1 {
            self.row_position -= 1;
        } else {
            self.buffer.chars.copy_within(1..BUFFER_HEIGHT, 0);
            self.clear_row(BUFFER_HEIGHT - 1);
        }

        self.column_position = 0;
    }

    #[inline]
    fn clear_row(&mut self, number_row: usize) {
        let blank: ScreenChar = ScreenChar {
            ascii_caracter: b' ',
            color_code: self.color_code
        };

        self.buffer.chars[number_row].fill(blank);
    }
}

impl Write for Writer {
    #[inline]
    fn write_str(&mut self, string: &str) -> Result {
        self.write_string(string);
        Ok(())
    }
}

#[doc(hidden)]
#[inline]
pub fn _print(args: Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
