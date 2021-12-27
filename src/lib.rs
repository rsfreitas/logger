
extern crate chrono;
extern crate colored;

pub mod fields;

use std::collections::HashMap;
use colored::Colorize;
use crate::fields::{FieldValue, RequiredFields};

pub struct Logger {
    required_fields: RequiredFields,
}

/// Logger is an interface that allows printing structured log messages into
/// the standard output.
impl Logger {
    /// new creates a new Logger facility allowing call the log API using it.
    /// It requires a RequiredFields struct that gathers information to be
    /// used at all messages.
    pub fn new(fields: &mut RequiredFields) -> Self {
        Logger{
            required_fields: fields.to_owned(),
        }
    }

    pub fn info(&self, msg: &str) {
        self.print(msg, "INFO", colored::Color::Blue, None)
    }

    pub fn infof(&self, msg: &str, fields: HashMap<String, FieldValue>) {
        self.print(msg, "INFO", colored::Color::Blue, Some(fields))
    }

    pub fn error(&self, msg: &str) {
        self.print(msg, "ERROR", colored::Color::Red, None)
    }

    pub fn errorf(&self, msg: &str, fields: HashMap<String, FieldValue>) {
        self.print(msg, "ERROR", colored::Color::Red, Some(fields))
    }

    pub fn debug(&self, msg: &str) {
        self.print(msg, "DEBUG", colored::Color::Green, None)
    }

    pub fn debugf(&self, msg: &str, fields: HashMap<String, FieldValue>) {
        self.print(msg, "DEBUG", colored::Color::Green, Some(fields))
    }

    pub fn warn(&self, msg: &str) {
        self.print(msg, "WARN", colored::Color::Yellow, None)
    }

    pub fn warnf(&self, msg: &str, fields: HashMap<String, FieldValue>) {
        self.print(msg, "WARN", colored::Color::Yellow, Some(fields))
    }

    fn print(&self, msg: &str, str_level: &str, color: colored::Color, fields: Option<HashMap<String, FieldValue>>) {
        let mut msg_fields = self.required_fields.data();

        // Appends user fields into the message required fields to be printed.
        if let Some(arg_fields) = fields {
            msg_fields.extend(arg_fields)
        }

        println!("{}\t{}\t{}\t{}", chrono::offset::Local::now().to_string(),
                                   str_level.color(color),
                                   msg,
                                   serde_json::to_string(&msg_fields).expect("could not parse required fields"));
    }
}

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
        let log = Logger::new(RequiredFields::new()
                              .add("key", FieldValue::String("value".to_string())));

        log.info("Hello world!");
        log.debug("Hello world!");
        log.error("Hello world!");
        log.warn("Hello world!");
    }

    #[test]
    pub fn test_log_with_empty_messages() {
    }

    #[test]
    pub fn test_log_with_fields() {
    }

    #[test]
    pub fn test_log_successful() {
    }
}

