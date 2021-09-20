use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightRed, Color::Black),
        buffer: unsafe { &mut *(0xB8000 as *mut Buffer) },
    });
}

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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }

    pub fn blinking(&mut self) -> ColorCode {
        ColorCode(self.0 | 1 << 7)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}


pub const BACKSPACE: u8 = 8;
pub const NEWLINE: u8 = b'\n' as u8;
pub const TABULATION: u8 = b'\t' as u8;

pub const GREET: &str = "@> ";

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            TABULATION => self.tabulate(),
            BACKSPACE => self.delete_byte(),
            NEWLINE => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                self.column_position += 1;

                self.cursor();
            }
        }
    }

    fn delete_byte(&mut self) {
        let blank = self.blank();

        if self.column_position > GREET.len() {
            self.column_position -= 1;
        }

        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;

        self.buffer.chars[row][col].write(blank);
        self.buffer.chars[row][col + 1].write(blank);
        self.cursor();
    }

    fn cursor(&mut self) {
        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;
        let color_code = self.color_code.blinking();

        self.buffer.chars[row][col].write(ScreenChar {
            ascii_character: b' ',
            color_code,
        });
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | NEWLINE | BACKSPACE | TABULATION => self.write_byte(byte),
                _ => self.write_byte(0xFE),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }

        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;
        let blank = self.blank();

        self.buffer.chars[row - 1][col].write(blank);
        
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
        self.greet();
    }

    fn clear_row(&mut self, row: usize) {
        let blank = self.blank();
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn blank(&mut self) -> ScreenChar {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        blank
    }

    fn tabulate(&mut self) {
        self.write_string("  ");
    }

    fn greet(&mut self) {
        self.write_string(GREET);
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
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

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";

    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}