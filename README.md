# logger

Logger is a rust crate for printing structured log messages into the standard
output.

## Example

The following example creates a logger facility with one required field, i.e.,
a field that will be printed in all messages.

```rust
extern crate logger;

use logger::builder::LoggerBuilder;
use logger::fields::FieldValue;

fn main() {
    let log = LoggerBuilder::default()
        .with_field("name", FieldValue::String("alert".to_string()))
        .build();

    log.info("Starting");
    log.debugf("Just a debug message", logger::fields!{
        "level" => FieldValue::String("critical".to_string()),
    });
}
```

This will output the following:

```bash
2021-12-24 12:17:03.346260 -03:00	INFO	Starting	{"name":"alert"}
2021-12-24 12:17:03.346952 -03:00	DEBUG	Just a debug message	{"name":"alert","level":"critical"}


