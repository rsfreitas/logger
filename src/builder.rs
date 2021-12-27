use crate::fields::{FieldValue, RequiredFields};
use crate::Logger;

/// Allows building a Logger interface using custom options.
#[derive(Debug, Clone)]
pub struct LoggerBuilder {
    pub(crate) env: Environment,
    pub(crate) required_fields: RequiredFields,

    ts_second_name: Option<String>,
    ts_millisecond_name: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Environment {
    Local,
    Test,
    Development,
    Production,
}

/// Allows building a Logger interface using custom options.
impl LoggerBuilder {
    const TIMESTAMP_SECOND_FIELD_NAME: &'static str = "local.ts";
    const TIMESTAMP_MILLISECOND_FIELD_NAME: &'static str = "local.ts_ms";

    /// Creates a new LoggerBuilder object.
    pub fn new() -> Self {
        LoggerBuilder {
            env: Environment::Local,
            required_fields: RequiredFields::default(),
            ts_second_name: None,
            ts_millisecond_name: None,
        }
    }

    /// Sets the logger environment as testing.
    pub fn as_test(mut self) -> Self {
        self.env = Environment::Test;
        self
    }

    /// Sets the logger environment as local.
    pub fn as_local(mut self) -> Self {
        self.env = Environment::Local;
        self
    }

    /// Sets the logger environment as development.
    pub fn as_dev(mut self) -> Self {
        self.env = Environment::Development;
        self
    }

    /// Sest the logger environment as production.
    pub fn as_prod(mut self) -> Self {
        self.env = Environment::Production;
        self
    }

    /// Changes the default log environment.
    pub fn with_env(&mut self, env: Environment) -> &mut Self {
        self.env = env;
        self
    }

    /// Adds a fixed field for all printed message.
    pub fn with_field(&mut self, key: &str, value: FieldValue) -> &mut Self {
        self.required_fields.add(key, value);
        self
    }

    /// Changes the names of timestamp fields that are appended into each
    /// log message.
    pub fn with_timestamp_field_name(&mut self, second: &str, millisecond: &str) -> &mut Self {
        self.ts_second_name = Some(second.to_string());
        self.ts_millisecond_name = Some(millisecond.to_string());
        self
    }

    /// Builds a Logger object with all defined options.
    pub fn build(&self) -> Logger {
        Logger::new(self)
    }

    pub(crate) fn has_color(&self) -> bool {
        matches!(self.env, Environment::Test | Environment::Local)
    }

    pub(crate) fn show_debug(&self) -> bool {
        matches!(self.env, Environment::Test | Environment::Local)
    }

    pub(crate) fn timestamp_second_field_name(&self) -> &str {
        match &self.ts_second_name {
            None => LoggerBuilder::TIMESTAMP_SECOND_FIELD_NAME,
            Some(s) => s,
        }
    }

    pub(crate) fn timestamp_millisecond_field_name(&self) -> &str {
        match &self.ts_millisecond_name {
            None => LoggerBuilder::TIMESTAMP_MILLISECOND_FIELD_NAME,
            Some(s) => s,
        }
    }
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
