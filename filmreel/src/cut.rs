use crate::utils::ordered_map;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Holds cut variables and their corresonding values stored in a series of key/value pairs.
///
/// [Cut Register](https://github.com/Bestowinc/filmReel/blob/supra_dump/cut.md#cut-register)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Register<'a> {
    #[serde(serialize_with = "ordered_map", borrow, flatten)]
    vars: Vars<'a>,
}

impl<'a> Register<'a> {
    pub fn new(val: Option<Value>) -> Self
    where
        Register<'a>: DeserializeOwned,
    {
        match val {
            Some(val) => serde_json::from_value(val).unwrap(),
            None => {
                let vars: Vars = HashMap::new();
                Self { vars }
            }
        }
    }
    pub fn insert(&mut self, key: &'a str, val: &'a str) -> Option<&'a str> {
        self.vars.insert(key, val)
    }
}

type Vars<'a> = HashMap<&'a str, &'a str>;

#[macro_export]
macro_rules! register {
    ({$( $key: expr => $val: expr ),*}) => {{
        let mut reg = Register::new(None);
        $(reg.insert($key, $val);)*
        reg
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut reg = register!({
            "FIRST_NAME"=> "Primus",
            "RESPONSE"=> "ALRIGHT"
        });
        reg.insert("LAST_NAME", "Secundus");
        assert_eq!(
            register!({
                "FIRST_NAME"=> "Primus",
                "RESPONSE"=> "ALRIGHT",
                "LAST_NAME"=> "Secundus"
            }),
            reg
        );
    }

    #[test]
    fn test_update() {
        let mut reg = register!({
            "FIRST_NAME"=> "Primus",
            "RESPONSE"=> "ALRIGHT"
        });
        reg.insert("FIRST_NAME", "Pietre");
        assert_eq!(
            register!({
                "FIRST_NAME"=> "Pietre",
                "RESPONSE"=> "ALRIGHT"
            }),
            reg
        );
    }
}

#[cfg(test)]
mod serde_tests {
    use super::*;
    use crate::test_ser_de;

    const REGISTER_JSON: &str = r#"
        {
          "FIRST_NAME": "Primus",
          "RESPONSE": "ALRIGHT"
        }
    "#;
    test_ser_de!(
        register_ser,
        register_de,
        register!({
            "FIRST_NAME"=> "Primus",
            "RESPONSE"=> "ALRIGHT"
        }),
        REGISTER_JSON
    );

    #[test]
    fn test_insert() {
        let mut reg = register!({
            "FIRST_NAME"=> "Primus",
            "RESPONSE"=> "ALRIGHT"
        });
        reg.insert("LAST_NAME", "Secundus");
        assert_eq!(
            register!({
                "FIRST_NAME"=> "Primus",
                "RESPONSE"=> "ALRIGHT",
                "LAST_NAME"=> "Secundus"
            }),
            reg
        );
    }
}
