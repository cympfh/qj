extern crate nom;
use nom::bytes::complete::is_not;
use nom::character::complete::char;
use nom::IResult;

extern crate serde_json;
use serde_json::Value;

use crate::address::Address;

/// Address + Value
#[derive(Debug, Clone)]
pub struct Expression {
    pub address: Address,
    pub value: Value,
}

impl Expression {
    pub fn parse(exp: &str) -> Expression {
        fn read_exp(exp: &str) -> IResult<&str, &str> {
            let (input, adr) = is_not("=")(exp)?;
            let (value, _) = char('=')(input)?;
            Ok((value, adr))
        }
        fn load_value(value: &str) -> Value {
            if let Ok(val) = serde_json::from_str(value) {
                val
            } else {
                Value::String(String::from(value))
            }
        }
        let (value, adr) = read_exp(exp).unwrap();
        Expression {
            address: Address::parse(adr),
            value: load_value(value),
        }
    }
}
