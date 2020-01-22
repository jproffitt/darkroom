use crate::utils::ordered_map;
use regex::Regex;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Holds cut variables and their corresonding values stored in a series of key/value pairs.
///
/// [Cut Register](https://github.com/Bestowinc/filmReel/blob/supra_dump/cut.md#cut-register)
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Register<'a> {
    #[serde(serialize_with = "ordered_map", borrow, flatten)]
    vars: Vars<'a>,
}

impl<'a> Register<'a> {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn insert(&mut self, key: &'a str, val: &'a str) -> Option<&'a str> {
        self.vars.insert(key, val)
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<&str, &str> {
        self.vars.iter()
    }
}

type Vars<'a> = HashMap<&'a str, &'a str>;

#[macro_export]
macro_rules! register {
    ({$( $key: expr => $val: expr ),*}) => {{
        use crate::cut::Register;

        let mut reg = Register::new();
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

    #[test]
    fn test_iter() {
        let reg = register!({
            "FIRST_NAME"=> "Primus",
            "RESPONSE"=> "ALRIGHT"
        });
        let mut kv_vec = vec![];

        for (k, v) in reg.iter() {
            kv_vec.push(k);
            kv_vec.push(v);
        }
        assert_eq!(
            vec![&"FIRST_NAME", &"Primus", &"RESPONSE", &"ALRIGHT"].sort(),
            kv_vec.sort()
        );
    }

    // #[test]
    // fn test_regex() {
    //     let re = Regex::new(
    //         r"(?x)
    //     (\\?) # escape character
    //     (\$\{) # leading brace
    //     (.+) # cut variable
    //     (}) # trailing brace
    //     ",
    //     )
    //     .unwrap();
    //     let test_string = "ok ${SOME_VAR}";
    //     let caps = re.captures(test_string).unwrap();
    //     assert_eq!(caps.get(0).unwrap().as_str(), "")
    // }
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
