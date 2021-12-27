extern crate chrono;
extern crate colored;

pub mod builder;
pub mod fields;

use crate::builder::LoggerBuilder;
use crate::fields::FieldValue;
use colored::Colorize;
use std::collections::HashMap;

/// Logger is an interface that allows printing structured log messages into
/// the standard output.
#[derive(Debug)]
pub struct Logger {
    builder: LoggerBuilder,
}

/// Logger is an interface that allows printing structured log messages into
/// the standard output.
impl Logger {
    /// Creates a new Logger facility allowing call the log API using it.
    /// Every message printed with a logger interface will have all builder
    /// defined key-value fields as well as the local timestamp.
    pub(crate) fn new(builder: &LoggerBuilder) -> Self {
        Logger {
            builder: builder.to_owned(),
        }
    }

    /// Prints a message into the standard output using the information
    /// level.
    pub fn info(&self, msg: &str) {
        self.print(msg, "INFO", colored::Color::Blue, None)
    }

    /// Prints a message into the standard output using the information
    /// level supporting the addition of fields, of key-value kind, into it.
    pub fn infof(&self, msg: &str, fields: HashMap<String, FieldValue>) {
        self.print(msg, "INFO", colored::Color::Blue, Some(fields))
    }

    /// Prints a message into the standard output using the error
    /// level.
    pub fn error(&self, msg: &str) {
        self.print(msg, "ERROR", colored::Color::Red, None)
    }

    /// Prints a message into the standard output using the error level
    /// supporting the addition of fields, of key-value kind, into it.
    pub fn errorf(&self, msg: &str, fields: HashMap<String, FieldValue>) {
        self.print(msg, "ERROR", colored::Color::Red, Some(fields))
    }

    /// Prints a message into the standard output using the debug
    /// level.
    pub fn debug(&self, msg: &str) {
        if self.builder.show_debug() {
            self.print(msg, "DEBUG", colored::Color::Green, None)
        }
    }

    /// Prints a message into the standard output using the debug level
    /// supporting the addition of fields, of key-value kind, into it.
    pub fn debugf(&self, msg: &str, fields: HashMap<String, FieldValue>) {
        if self.builder.show_debug() {
            self.print(msg, "DEBUG", colored::Color::Green, Some(fields))
        }
    }

    /// Prints a message into the standard output using the warning
    /// level.
    pub fn warn(&self, msg: &str) {
        self.print(msg, "WARN", colored::Color::Yellow, None)
    }

    /// Prints a message into the standard output using the warning level
    /// supporting the addition of fields, of key-value kind, into it.
    pub fn warnf(&self, msg: &str, fields: HashMap<String, FieldValue>) {
        self.print(msg, "WARN", colored::Color::Yellow, Some(fields))
    }

    fn print(
        &self,
        msg: &str,
        str_level: &str,
        color: colored::Color,
        fields: Option<HashMap<String, FieldValue>>,
    ) {
        let mut msg_fields = self.builder.required_fields.data();
        let now = chrono::offset::Local::now();

        // Adds fields that (may) change with each message.
        msg_fields.insert(
            self.builder.timestamp_second_field_name().to_string(),
            FieldValue::Number(now.timestamp()),
        );
        msg_fields.insert(
            self.builder.timestamp_millisecond_field_name().to_string(),
            FieldValue::Number(now.timestamp_millis()),
        );

        // Appends user fields into the message required fields to be printed.
        if let Some(arg_fields) = fields {
            msg_fields.extend(arg_fields)
        }

        println!(
            "{}\t{}\t{}\t{}",
            now.to_string(),
            match self.builder.has_color() {
                true => str_level.color(color),
                false => str_level.clear(),
            },
            msg,
            serde_json::to_string(&msg_fields).expect("could not parse required fields")
        );
    }
}

/// This macros allows creating one or more key-value fields to be added into
/// a log message.
#[macro_export]
macro_rules! fields {
    ($($k:expr => $v:expr),* $(,)?) => {
        {
            use std::iter::{Iterator, IntoIterator};
            Iterator::collect(IntoIterator::into_iter([$(($k.to_string(), $v),)*]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_new_logger() {
        let log = LoggerBuilder::default().build();

        log.info("Hello world!");
        log.debug("Hello world!");
        log.error("Hello world!");
        log.warn("Hello world!");
    }

    #[test]
    pub fn test_new_logger_with_fields() {
        let log = LoggerBuilder::default()
            .with_field("name", FieldValue::String("example".to_string()))
            .build();

        log.info("Hello world!");
        log.debug("Hello world!");
        log.error("Hello world!");
        log.warn("Hello world!");
    }

    #[test]
    pub fn test_log_with_empty_messages() {}

    #[test]
    pub fn test_log_with_fields() {}

    #[test]
    pub fn test_log_successful() {}
}
