use crate::{cut::Register, error::FrError, frame::Frame};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct VirtualReel<'a> {
    name:   &'a str,
    // use BTtreeMap en lieu of HashMap to maintain order
    #[serde(borrow)]
    frames: BTreeMap<&'a str, VirtualFrame<'a>>,
    cut:    VirtualCut<'a>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum VirtualCut<'a> {
    #[serde(borrow)]
    MergeCuts(Vec<&'a str>),
    Register(Register),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
enum VirtualFrame<'a> {
    File(PathBuf),
    #[serde(borrow)]
    Frame(Frame<'a>),
}

#[macro_export]
macro_rules! frames {
    ({$( $key: expr => $val: expr ),*}) => {{
        use $crate::frame::Frame;
        use ::std::collections::BTreeMap;

        let mut map: BTreeMap<&str, VirtualFrame> = BTreeMap::new();
        $(map.insert($key, VirtualFrame::Frame($val));)*
        map
    }}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_ser_de;
    use rstest::*;
    const VREEL_JSON: &str = r#"
{
  "name": "reel_name",
  "frames": {
    "1": {
      "protocol": "HTTP",
      "request": {
        "uri": "POST /logout/${USER_ID}"
      },
      "response": {
        "status": 200
      }
    }
  },
  "cut": {"KEY": "value"}
}
    "#;

    const SIMPLE_FRAME_JSON: &str = r#"
{
  "protocol": "HTTP",
  "request": {
    "uri": "POST /logout/${USER_ID}"
  },
  "response": {
    "status": 200
  }
}
    "#;

    test_ser_de!(
        vframe,
        VirtualReel {
            name:   "reel_name",
            frames: frames!({"1" => Frame::new(SIMPLE_FRAME_JSON).unwrap()}),
            cut:    VirtualCut::Register(register!({"KEY" => "value})),
        },
        VREEL_JSON
    );
}
