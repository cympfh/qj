extern crate nom;
use nom::bytes::complete::is_not;
use nom::character::complete::{char, digit1};
use nom::IResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressEntity {
    Field(String),
    Index(usize),
}

use AddressEntity::{Field, Index};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub data: Vec<AddressEntity>,
}

impl Address {
    pub fn parse(adr: &str) -> Self {
        let mut data = vec![];

        if adr == "." {
            return Address { data };
        }

        fn field(input: &str) -> IResult<&str, &str> {
            let (input, _) = char('.')(input)?;
            let (rest, field) = is_not(".[]")(input)?;
            Ok((rest, field))
        }

        fn index(input: &str) -> IResult<&str, &str> {
            let (input, _) = char('[')(input)?;
            let (input, digits) = digit1(input)?;
            let (rest, _) = char(']')(input)?;
            Ok((rest, digits))
        }

        let mut adr = adr;
        while adr.len() > 0 {
            if let Ok((rest, field)) = field(adr) {
                data.push(Field(String::from(field)));
                adr = rest;
                continue;
            }
            if let Ok((rest, index)) = index(adr) {
                let index = index.parse::<usize>().unwrap();
                data.push(Index(index));

                adr = rest;
                continue;
            }
            panic!(format!("Cannot parse: {}", adr));
        }

        Address { data }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use AddressEntity::{Field, Index};

    #[test]
    fn test_parse() {
        assert_eq!(Address::parse("."), Address { data: vec![] });
        assert_eq!(
            Address::parse(".x"),
            Address {
                data: vec![Field(String::from("x"))]
            }
        );
        assert_eq!(
            Address::parse(".x.y"),
            Address {
                data: vec![Field(String::from("x")), Field(String::from("y"))]
            }
        );
        assert_eq!(
            Address::parse(".x[1][2].y"),
            Address {
                data: vec![
                    Field(String::from("x")),
                    Index(1),
                    Index(2),
                    Field(String::from("y"))
                ]
            }
        );
    }
}
