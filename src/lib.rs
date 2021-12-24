
extern crate chrono;

pub mod fields;

use std::collections::HashMap;
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
        self.print(msg, "INFO", None)
    }

    pub fn error(&self, msg: &str) {
        self.print(msg, "ERROR", None)
    }

    pub fn debug(&self, msg: &str) {
        self.print(msg, "DEBUG", None)
    }

    pub fn debugf(&self, msg: &str, fields: HashMap<String, FieldValue>) {
        self.print(msg, "DEBUG", Some(fields))
    }

    pub fn warn(&self, msg: &str) {
        self.print(msg, "WARN", None)
    }

    fn print(&self, msg: &str, str_level: &str, fields: Option<HashMap<String, FieldValue>>) {
        let mut msg_fields = self.required_fields.data();
        if let Some(arg_fields) = fields {
            msg_fields.extend(arg_fields)
        }

        println!("{}\t{}\t{}\t{}", chrono::offset::Local::now().to_string(),
                                   str_level,
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

