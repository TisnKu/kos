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

macro_rules! logger_template {
    ($t:ident, $color: expr) => {
        #[macro_export]
        macro_rules! $t {
            ($args:tt) => ({
                $crate::console::print(format_args!("\x1b[{}m[{}]: {}\x1b[0m\n", $color as u8, stringify!($t), format_args!($args)));
            })
        }
    }
}

logger_template!(info, console::Logger::INFO);
logger_template!(warn, console::Logger::WARN);
logger_template!(debug, console::Logger::DEBUG);
logger_template!(trace, console::Logger::TRACE);
logger_template!(error, console::Logger::ERROR);