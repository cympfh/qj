mod address;
use address::AddressEntity;

mod expression;
use expression::Expression;

extern crate serde_json;
use serde_json::{Map, Value};

extern crate structopt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "e", long = "exp")]
    exps: Vec<String>,
}

fn set(val: &mut Value, exp: Expression) {
    let mut ptr = val;
    let len = exp.address.data.len();
    if len == 0 {
        *ptr = exp.value;
        return;
    }
    for i in 0..len - 1 {
        match (&exp.address.data[i], &exp.address.data[i + 1]) {
            (AddressEntity::Field(f), AddressEntity::Field(_)) => {
                if ptr[f] == Value::Null {
                    ptr[f] = Value::Object(Map::new());
                }
                ptr = &mut ptr[f];
            }
            (AddressEntity::Field(f), AddressEntity::Index(_)) => {
                if ptr[f] == Value::Null {
                    ptr[f] = Value::Array(vec![]);
                }
                ptr = &mut ptr[f];
            }
            (AddressEntity::Index(i), AddressEntity::Field(_)) => {
                {
                    let arr = ptr.as_array_mut().unwrap();
                    while arr.len() <= *i {
                        arr.push(Value::Null);
                    }
                }
                if ptr[i] == Value::Null {
                    ptr[i] = Value::Object(Map::new());
                }
                ptr = &mut ptr[i];
            }
            (AddressEntity::Index(i), AddressEntity::Index(_)) => {
                {
                    let arr = ptr.as_array_mut().unwrap();
                    while arr.len() <= *i {
                        arr.push(Value::Null);
                    }
                }
                if ptr[i] == Value::Null {
                    ptr[i] = Value::Array(vec![]);
                }
                ptr = &mut ptr[i];
            }
        }
    }
    match &exp.address.data[len - 1] {
        AddressEntity::Field(f) => {
            ptr[f] = exp.value;
        }
        AddressEntity::Index(i) => {
            {
                let arr = ptr.as_array_mut().unwrap();
                while arr.len() <= *i {
                    arr.push(Value::Null);
                }
            }
            ptr[i] = exp.value;
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    let mut ret = Value::Null;
    for exp in opt.exps.iter() {
        let exp = Expression::parse(exp);
        set(&mut ret, exp);
    }
    println!("{}", serde_json::to_string(&ret).unwrap());
}
