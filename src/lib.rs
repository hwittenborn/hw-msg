#![cfg_attr(docsrs, feature(doc_cfg))]
//! This crate provides macros for the style of CLI messages I like to use in my programs.
//!
//! The base macros are:
//! - [`info`] - Print an info-styled message
//! - [`warning`] - Print a warning-styled message
//! - [`error`] - Print an error-styled message
//! - [`question`] - Print a question-styled message
//! - [`debug`] - Print a debug-styled message
//!
//! These macros will print the message **without** a trailing newline, like how [`print`] does. Use the `*ln` variants of each of these macros to get [`println`]-like functionality (i.e. [`infoln`]).
//!
//! The `*_fmt` variants work like the base macros, but return the string instead of printing it.
use colored::{ColoredString, Colorize};

#[cfg(feature = "logging")]
mod logging;
#[cfg_attr(docsrs, doc(cfg(feature = "logging")))]
#[cfg(feature = "logging")]
pub use logging::HwLogger;

#[doc(hidden)]
pub fn get_info<T: ToString>(string: T) -> ColoredString {
    string.to_string().cyan().bold()
}

#[doc(hidden)]
pub fn get_warning<T: ToString>(string: T) -> ColoredString {
    string.to_string().yellow().bold()
}

#[doc(hidden)]
pub fn get_error<T: ToString>(string: T) -> ColoredString {
    string.to_string().red().bold()
}

#[doc(hidden)]
pub fn get_question<T: ToString>(string: T) -> ColoredString {
    string.to_string().magenta().bold()
}

#[doc(hidden)]
pub fn get_debug<T: ToString>(string: T) -> ColoredString {
    string.to_string().green().bold()
}

#[macro_export]
macro_rules! info_fmt {
    ($($arg:tt)*) => {{
        format!("{} ", $crate::get_info("Info:")) + &format!($($arg)*)
    }}
}

#[macro_export]
macro_rules! warning_fmt {
    ($($arg:tt)*) => {{
        format!("{} ", $crate::get_warning("Warning:")) + &format!($($arg)*)
    }}
}

#[macro_export]
macro_rules! error_fmt {
    ($($arg:tt)*) => {{
        format!("{} ", $crate::get_error("Error:")) + &format!($($arg)*)
    }}
}

#[macro_export]
macro_rules! question_fmt {
    ($($arg:tt)*) => {{
        format!("{} ", $crate::get_question("Question:")) + &format!($($arg)*)
    }}
}

#[macro_export]
macro_rules! debug_fmt {
    ($($arg:tt)*) => {{
        format!("{} ", $crate::get_debug("Debug:")) + &format!($($arg)*)
    }}
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        print!("{}", $crate::info_fmt!($($arg)*))
    }}
}

#[macro_export]
macro_rules! infoln {
    ($($arg:tt)*) => {{
        $crate::info!($($arg)*);
        println!()
    }}
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{
        print!("{}", $crate::warning_fmt!($($arg)*))
    }}
}

#[macro_export]
macro_rules! warningln {
    ($($arg:tt)*) => {{
        $crate::warning!($($arg)*);
        println!()
    }}
}
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        print!("{}", $crate::error_fmt!($($arg)*))
    }}
}

#[macro_export]
macro_rules! errorln {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*);
        println!()
    }}
}
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        print!("{}", $crate::debug_fmt!($($arg)*))
    }}
}

#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => {{
        $crate::debug!($($arg)*);
        println!()
    }}
}

#[macro_export]
macro_rules! question {
    ($($arg:tt)*) => {{
        print!("{}", $crate::question_fmt!($($arg)*))
    }}
}
