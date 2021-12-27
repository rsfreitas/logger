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
2021-12-27 11:56:28.282117 -03:00	INFO	Starting	{"name":"alert","local.ts":1640616988,"local.ts_ms":1640616988282}
2021-12-27 11:56:28.283648 -03:00	DEBUG	Just a debug message	{"name":"alert","local.ts":1640616988,"local.ts_ms":1640616988283,"level":"critical"
```

## Features

Logger supports categorizing messages in some levels. Also, when choosing `Local`
or `Test` environments, it uses different colors on the level information.

The current supported levels (and their terminal colors) are the following:

* Debug (green)
* Info (blue)
* Warning (yellow)
* Error (red)

