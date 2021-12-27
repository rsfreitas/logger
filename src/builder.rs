use crate::fields::{FieldValue, RequiredFields};
use crate::Logger;

/// Allows building a Logger interface using custom options.
#[derive(Clone)]
pub struct LoggerBuilder {
    pub(crate) env: Environment,
    pub(crate) required_fields: RequiredFields,
}

#[derive(Clone)]
pub enum Environment {
    Local,
    Test,
    Development,
    Production,
}

/// Allows building a Logger interface using custom options.
impl LoggerBuilder {
    /// Creates a new LoggerBuilder object.
    pub fn new() -> Self {
        LoggerBuilder {
            env: Environment::Local,
            required_fields: RequiredFields::default(),
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

    /// Adds a fixed field for all printed message.
    pub fn with_fielf(&mut self, key: &str, value: FieldValue) -> &mut Self {
        self.required_fields.add(key, value);
        self
    }

    /// Builds a Logger object with all defined options.
    pub fn build(self) -> Logger {
        Logger::new(self)
    }

    pub(crate) fn has_color(&self) -> bool {
        matches!(self.env, Environment::Test | Environment::Local)
    }

    pub(crate) fn show_debug(&self) -> bool {
        matches!(self.env, Environment::Test | Environment::Local)
    }
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
