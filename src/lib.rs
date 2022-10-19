use colored::{ColoredString, Colorize};

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

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        print!("{} ", $crate::get_info("Info:"));
        print!($($arg)*);

    }
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        print!("{} ", $crate::get_warning("Warning:"));
        print!($($arg)*);

    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        print!("{} ", $crate::get_error("Error:"));
        print!($($arg)*);

    }
}

#[macro_export]
macro_rules! question {
    ($($arg:tt)*) => {
        print!("{} ", $crate::get_question("Question:"));
        print!($($arg)*);

    }
}
