use core::ptr::Unique;

use spin::Mutex;

#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    row_position: usize,
    column_position: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

/*
 * This struct handles everything about writing to the VGA buffer (line wrapping
 * et al.).
 *
 */

impl Writer {
    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
          self.write_byte(byte);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                self.buffer().chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };
                self.column_position += 1;
            }
        }
    }

    pub fn reset_positions(&mut self) {
        self.column_position = 0;
        self.row_position = 0;
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe{ self.buffer.get_mut() }
    }

    fn new_line(&mut self) {
        if self.row_position > BUFFER_HEIGHT - 2 {
            self.row_position = BUFFER_HEIGHT - 1;
            for row in 0..(BUFFER_HEIGHT-1) {
                let buffer = self.buffer();
                buffer.chars[row] = buffer.chars[row + 1];
            }
            self.clear_row(BUFFER_HEIGHT-1);
        } else {
            self.row_position += 1;
        }
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        self.buffer().chars[row] = [blank; BUFFER_WIDTH];
    }
}

// Required for formatting macro support

impl ::core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
          self.write_byte(byte)
        }
        Ok(())
    }
}

// Wraps writer in a basic spinlock mutex to make it safe.

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    row_position: 0,
    column_position: 0,
    color_code: ColorCode::new(Color::White, Color::Black),
    buffer: unsafe { Unique::new(0xb8000 as *mut _) },
});

/*
 * Macros to replace the built-in print! and println!
 *
 */

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
            use core::fmt::Write;
            let mut writer = $crate::vga_buffer::WRITER.lock();
            writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

// Does what it says on the tin
pub fn clear_screen() {
    for _ in 0..BUFFER_HEIGHT {
        for _ in 0..BUFFER_WIDTH {
            print!(" ");
        }
        print!("\n");
    }
    let mut writer = WRITER.lock();
    writer.reset_positions();
}
