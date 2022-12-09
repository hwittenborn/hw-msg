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
macro_rules! info_fmt {
    ($($arg:tt)*) => {
        format!("{} ", $crate::get_info("Info:")) + &format!($($arg)*)
    }
}

#[macro_export]
macro_rules! warning_fmt {
    ($($arg:tt)*) => {
        format!("{} ", $crate::get_warning("Warning:")) + &format!($($arg)*)
    }
}

#[macro_export]
macro_rules! error_fmt {
    ($($arg:tt)*) => {
        format!("{} ", $crate::get_error("Error:")) + &format!($($arg)*)
    }
}

#[macro_export]
macro_rules! question_fmt {
    ($($arg:tt)*) => {
        format!("{} ", $crate::get_question("Question:")) + &format!($($arg)*)
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        print!("{}", $crate::info_fmt!($($arg)*));
    }
}

#[macro_export]
macro_rules! infoln {
    ($($arg:tt)*) => {
        $crate::info!($($arg)*);
        println!();
    }
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        print!("{}", $crate::warning_fmt!($($arg)*));
    }
}

#[macro_export]
macro_rules! warningln {
    ($($arg:tt)*) => {
        $crate::warning!($($arg)*);
        println!();
    }
}
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        print!("{}", $crate::error_fmt!($($arg)*));

    }
}

#[macro_export]
macro_rules! errorln {
    ($($arg:tt)*) => {
        $crate::error!($($arg)*);
        println!();
    }
}
#[macro_export]
macro_rules! question {
    ($($arg:tt)*) => {
        print!("{}", $crate::question_fmt!($($arg)*));
    }
}
