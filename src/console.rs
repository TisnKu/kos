use core::fmt;
use core::fmt::Write;

use crate::sbi::console_putchar;

pub enum Logger {
    ERROR = 31,
    WARN = 93,
    INFO = 34,
    DEBUG = 32,
    TRACE = 90,
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            console_putchar(c as u8);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap()
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
       println!("\x1b[{}m[INFO]: {}\x1b[0m\n", console::Logger::INFO as u8, format_args!($($arg)*));
    }
}
#[macro_export]
macro_rules! debug {
    ($($arg: tt)*) => {
       println!("\x1b[{}m[DEBUG]: {}\x1b[0m\n", console::Logger::DEBUG as u8, format_args!($($arg)*));
    }
}
#[macro_export]
macro_rules! error {
    ($($arg: tt)*) => {
       println!("\x1b[{}m[ERROR]: {}\x1b[0m\n", console::Logger::ERROR as u8, format_args!($($arg)*));
    }
}
#[macro_export]
macro_rules! trace {
    ($($arg: tt)*) => {
       println!("\x1b[{}m[TRACE]: {}\x1b[0m\n", console::Logger::TRACE as u8, format_args!($($arg)*));
    }
}
#[macro_export]
macro_rules! warn {
    ($($arg: tt)*) => {
       println!("\x1b[{}m[WARN]: {}\x1b[0m\n", console::Logger::WARN as u8, format_args!($($arg)*));
    }
}