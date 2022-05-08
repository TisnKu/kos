use core::fmt;
use core::fmt::{Arguments, Write};

use log::{Level, LevelFilter, Log, Metadata, Record};

use crate::sbi::console_putchar;

pub enum LogColor {
    ERROR = 31,
    WARN = 93,
    INFO = 34,
    DEBUG = 32,
    TRACE = 90,
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
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
macro_rules! with_color {
    ($args:ident, $color:ident) => {
       format_args!("\x1b[{}m{}\x1b[0m\n", $color as u8, $args)
    }
}

pub fn init_logger() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}

fn get_color(log_level: Level) -> LogColor {
    match log_level {
        Level::Error => LogColor::ERROR,
        Level::Warn => LogColor::WARN,
        Level::Info => LogColor::INFO,
        Level::Debug => LogColor::DEBUG,
        Level::Trace => LogColor::TRACE,
    }
}

fn print_with_color(args: Arguments, color: LogColor) {
    Stdout.write_fmt(with_color!(args, color)).unwrap();
}

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        println!("hello world");
        print_with_color(
            format_args!("[{:>5}]: {}", record.level(), record.args()),
            get_color(record.level()));
    }

    fn flush(&self) {}
}