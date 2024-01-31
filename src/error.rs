use core::fmt;
use serde_json::Value;
use std::any::Any;

#[derive(Debug)]
pub enum Error {
    DeserializatonError {
        message: String,
        de_str: Option<String>,
        value: Option<Value>,
        nested_error: Option<Box<dyn std::error::Error + Send>>,
    },
    FromStrError {
        message: String,
        string: String,
        nested_error: Option<Box<dyn std::error::Error + Send>>,
    },
    GenericError {
        message: String,
        nested_error: Option<Box<dyn std::error::Error + Send>>,
    },
    JoinError {
        message: String,
        nested_error: Option<Box<dyn Any + Send>>,
    },
    IOError {
        nested_error: std::io::Error,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        match self {
            Error::DeserializatonError {
                message,
                de_str,
                value,
                nested_error,
            } => {
                s.push_str(&format!("{}\n", message));
                append_option(&mut s, de_str, "Deserialization string");
                append_option(&mut s, value, "Value");
                append_option(&mut s, nested_error, "Nested error");
            }
            Error::FromStrError {
                message,
                string,
                nested_error,
            } => {
                s.push_str(&format!("{}\n", message));
                s.push_str(&format!("String: {}\n", string));
                append_option(&mut s, nested_error, "Nested error");
            }
            Error::GenericError {
                message,
                nested_error,
            } => {
                s.push_str(&format!("{}\n", message));
                append_option(&mut s, nested_error, "Nested error");
            }
            Error::JoinError {
                message,
                nested_error,
            } => {
                s.push_str(&format!("{}\n", message));
                if let Some(e) = nested_error {
                    s.push_str(&format!("Nested Error: {:?}\n", e));
                }
            }
            Error::IOError {
                nested_error: io_error,
            } => {
                s.push_str(&format!("IO Error: {}\n", io_error));
            }
        }
        return write!(f, "{}", s);
    }
}

impl std::error::Error for Error {}

fn append_option<T>(s: &mut String, op: &Option<T>, prefix: &str)
where
    T: fmt::Display,
{
    if let Some(val) = op {
        s.push_str(&format!("{}: {}\n", prefix, val));
    }
}
