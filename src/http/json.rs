#![allow(unused)]
use serde_json::{Value, json};
use crate::http::types;

pub fn get_string(k: &Value, name: String, default: String) -> String {
    match k.get(name.as_str()) {
        Some(x) => {
            match x.as_str() {
                Some (y) => {
                    return y.to_string();
                },
                None => {}
            }
        },
        None => {}
    }

    match k.get(name.as_str()) {
        Some(x) => {
            match x.as_bool() {
                Some (y) => {
                    return y.to_string();
                },
                None => {}
            }
        },
        None => {}
    }

    match k.get(name.as_str()) {
        Some(x) => {
            match x.as_i64() {
                Some (y) => {
                    return y.to_string();
                },
                None => {}
            }
        },
        None => {}
    }

    match k.get(name.as_str()) {
        Some(x) => {
            match x.as_f64() {
                Some (y) => {
                    return y.to_string();
                },
                None => {}
            }
        },
        None => {}
    }

    match k.get(name.as_str()) {
        Some(x) => {
            match x.as_u64() {
                Some (y) => {
                    return y.to_string();
                },
                None => {}
            }
        },
        None => {}
    }

    match k.get(name.as_str()) {
        Some(x) => {
            match x.as_object() {
                Some (y) => {
                    return format!("{:?}", y);
                },
                None => {}
            }
        },
        None => {}
    }
    return default;
}

pub fn get_value(key: String, o: &Value, v: String) -> Value {
    let mut value = Value::Null;
    if o.is_boolean() {
        if v.to_lowercase() == "true".to_string() {
            value = Value::Bool(true);
        } else if v.to_lowercase() == "false".to_string() {
            value = Value::Bool(false);
        } else {
            println!("Invalid bool value {{ {} : {} }}", key.as_str(), v.as_str());
            return value;
        }
    } else if o.is_f64() {
        let (b, f) = types::str_to_f64(v.clone());
        if b == false {
            println!("Invalid f64 value {{ {} : {} }}", key.as_str(), v.as_str());
            return value;
        }
        value = json!(f);
    } else if o.is_i64() || o.is_number() {
        let (b, f) = types::str_to_i64(v.clone());
        if b == false {
            println!("Invalid i64 value {{ {} : {} }}", key.as_str(), v.as_str());
            return value;
        }
        value = json!(f);
    } else if o.is_string() {
        value = json!(v.as_str());
    } else if o.is_u64() {
        let (b, f) = types::str_to_u64(v.clone());
        if b == false {
            println!("Invalid u64 value {{ {} : {} }}", key.as_str(), v.as_str());
            return value;
        }
        value = json!(f);
    }
    return value;
}

pub fn value_to_string(value: &Value) -> String {
    let psx = value.as_str();
    let mut ps = String::from("");
    match psx {
        Some(p) => { ps = p.to_string(); },
        None => {}
    }
    return ps;
}

pub fn value_to_arr_string(value: &Value) -> Vec<String> {
    let psx = value.as_array();
    let mut ps = vec![];
    match psx {
        Some(p) => { 
            for p2 in p {
                ps.push(value_to_string(p2));
            }
        },
        None => {}
    }
    return ps;
}

pub fn value_to_i32(value: &Value) -> i32 {
    let url_indexx = value.as_i64();
    let mut url_index = 3;
    match url_indexx {
        Some(ss) => { url_index = ss as i32; },
        None => {}
    }
    return url_index;
}

pub fn value_to_bool(value: &Value) -> bool {
    let psx = value.as_bool();
    let mut ps = false;
    match psx {
        Some(p) => { ps = p; },
        None => {}
    }
    return ps;
}