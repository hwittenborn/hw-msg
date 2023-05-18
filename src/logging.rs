use log::{Level, Metadata, Record};

/// This struct allows you to use the
/// [`log` crate's macros](https://docs.rs/log/latest/log/index.html#usage)
/// instead of the ones provided by this crate.
///
/// # Example
///
/// ```rust
/// use hw_msg::logging::HwLogger;
///
/// log::set_logger(&hw_msg::HwLogger).unwrap();
/// log::info!("We're using `hw_msg` from the `log` crate!");
/// ```
///
/// **Note**: This struct is currently incompatible with the [`log::trace`] macro, and will panic if
/// you call it with this logger.
pub struct HwLogger;

impl log::Log for HwLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if let Some(level) = log::max_level().to_level() {
            metadata.level() <= level
        } else {
            false
        }
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let args = record.args();

        match record.level() {
            Level::Error => crate::errorln!("{args}"),
            Level::Warn => crate::warningln!("{args}"),
            Level::Info => crate::infoln!("{args}"),
            Level::Debug => crate::debugln!("{args}"),
            Level::Trace => {
                unimplemented!("`Level::Trace` is unsupported for the `HwLogger` logger")
            }
        }
    }

    fn flush(&self) {}
}
