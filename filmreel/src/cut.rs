use serde::{Deserialize, Serialize, Serializer};
use serde_json;
use std::collections::{BTreeMap, HashMap};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Register<'a> {
    #[serde(flatten)]
    #[serde(serialize_with = "ordered_map", borrow)]
    vars: Vars<'a>,
}

pub type Vars<'a> = HashMap<&'a str, String>;
impl<'a> Register<'a> {
    pub fn new() -> Self {
        let vars: Vars = HashMap::new();
        Self { vars }
    }
}

#[macro_export]
macro_rules! register {
    ($( $key: expr => $val: expr ),*) => {{
         let mut reg = Register::new();
         $( reg.vars.insert($key, String::from($val)); )*
         reg
    }}
}

pub fn ordered_map<S>(value: &Vars, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

#[cfg(test)]
mod tests {
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
        Register,
        register![
          "FIRST_NAME"=> "Primus",
          "RESPONSE"=> "ALRIGHT"],
        REGISTER_JSON
    );
}
