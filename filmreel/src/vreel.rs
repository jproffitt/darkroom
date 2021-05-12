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

impl<'a> From<PathBuf> for VirtualFrame<'a> {
    fn from(p: PathBuf) -> Self {
        Self::File(p)
    }
}

impl<'a> From<Frame<'a>> for VirtualFrame<'a> {
    fn from(f: Frame<'a>) -> Self {
        Self::Frame(f)
    }
}

#[macro_export]
macro_rules! frames {
    ({$( $key: expr => $val: expr ),*}) => {{
        use ::std::collections::BTreeMap;

        let mut map: BTreeMap<&str, VirtualFrame> = BTreeMap::new();
        $(map.insert($key, VirtualFrame::from($val));)*
        map
    }}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{register, test_ser_de};

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
            cut:    VirtualCut::Register(register!({"KEY" => "value"})),
        },
        VREEL_JSON
    );

    const PATH_VREEL_JSON: &str = r#"
{
  "name": "reel_name",
  "frames": {
    "1": "other_reel.01s.name.fr.json"
  },
  "cut": ["other_reel.cut.json"]
}
    "#;

    test_ser_de!(
        pathbuf_vframe,
        VirtualReel {
            name:   "reel_name",
            frames: frames!({"1" => PathBuf::from("other_reel.01s.name.fr.json")}),
            cut:    VirtualCut::MergeCuts(vec!["other_reel.cut.json"]),
        },
        PATH_VREEL_JSON
    );
}
