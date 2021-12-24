# logger

Logger is a rust crate for printing structured log messages into the standard
output.

## Example

The following example creates a logger facility with one required field, i.e.,
a field that will be printed in all messages.

{{{
extern crate logger;

use logger::Logger;
use logger::fields::{FieldValue, RequiredFields};

fn main() {
    let log = Logger::new(RequiredFields::new().
                          add("name", FieldValue::String("alert".to_string())));

    log.info("Starting");
    log.debugf("Just a debug message", logger::fields!{
        "level" => FieldValue::String("critical".to_string()),
    });
}
}}}

This will output the following:

{{{
2021-12-24 12:17:03.346260 -03:00	INFO	Starting	{"name":"alert"}
2021-12-24 12:17:03.346952 -03:00	DEBUG	Just a debug message	{"name":"alert","level":"critical"}
}}}

