// Filters are functions with the
// fn(Value, HashMap<String, Value>) -> Result<Value>
// definition
//
// Register with:
// t.register_filter("do_nothing", do_nothing_filter);
// t.register_filter("upper", string::upper);

use std::collections::HashMap;

use tera::Result;
use tera::Value;

pub fn underscored(val: &Value, _context: &HashMap<String, Value>) -> Result<Value> {
    match val {
        Value::String(s) => Ok(Value::String(s.replace("-", "_"))),
        _ => Ok(val.to_owned()),
    }
}

pub fn shielded(val: &Value, _context: &HashMap<String, Value>) -> Result<Value> {
    if let Some(v) = val.as_str() {
        Ok(Value::String(
            v.replace("-", "--").replace("_", "__").replace(" ", "_"),
        ))
    } else {
        Ok(val.to_owned())
    }
}
