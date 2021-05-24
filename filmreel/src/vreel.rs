use crate::{cut::Register, error::FrError, frame::Frame};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::BTreeMap, convert::TryFrom, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct VirtualReel<'a> {
    pub name:   Cow<'a, str>,
    // use BTtreeMap en lieu of HashMap to maintain order
    pub frames: VirtualFrames<'a>,
    pub cut:    VirtualCut,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum VirtualCut {
    MergeCuts(Vec<PathBuf>),
    Cut(PathBuf),
    Register(Register),
}

/// VirtualFrames represents the frames variant containing a list of frames that can be renamed
///
/// VirtualFrames::ReanamedList variant will replace the frame name with the key value when running the
/// VirualReel (ordering reel flow by the new key name):
///
///  ```json
///  {"new_frame_name": "usr.01s.createuser.fr.json"}
///  ```
///  The example above will run `"usr.01s.createuser.fr.json"` as `"new_frame_name"`
///
/// VirtualFrames::List variant will retain the frame name and order the reel sequence by the
/// index position of the filepath:
///
///  ```json
///  ["usr.04s.validateuser.fr.json", "usr.01s.createuser.fr.json"]
///  ```
///
///  The example above will run `usr.01s.createuser.fr.json` *after* `usr.04s.validateuser.fr.json`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum VirtualFrames<'a> {
    RenamedList(BTreeMap<Cow<'a, str>, PathBuf>),
    List(Vec<PathBuf>),
}

impl<'a> TryFrom<PathBuf> for VirtualReel<'a> {
    type Error = FrError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let buf = crate::file_to_reader(path).map_err(|e| FrError::File(e.to_string()))?;
        let vreel = serde_json::from_reader(buf)?;
        Ok(vreel)
    }
}
//     () => (
//         $crate::__rust_force_expr!($crate::vec::Vec::new())
//     );
//     ($elem:expr; $n:expr) => (
//         $crate::__rust_force_expr!($crate::vec::from_elem($elem, $n))
//     );
//     ($($x:expr),+ $(,)?) => (
//         $crate::__rust_force_expr!(<[_]>::into_vec(box [$($x),+]))
//     );

#[macro_export]
macro_rules! frames {
    ([$val: expr]) => (
        use ::std::path::PathBuf;
        VirtualFrames::List(vec![PathBuf::from($val)])
    );
    ([$($val: expr),+]) => ({
        use ::std::path::PathBuf;

        let mut vec = Vec::new();
        // vec.insert(PathBuf::from($val));
        $(vec.push(PathBuf::from($val));)*
        VirtualFrames::List(vec)
    });
    ({$( $key: expr => $val: expr ),*}) => {{
        use ::std::collections::BTreeMap;
        use ::std::path::PathBuf;

        let mut map =  BTreeMap::new();
        $(map.insert($key.into(), $val);)*
            VirtualFrames::RenamedList(map)
    }}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{register, test_ser_de};

    const VREEL_JSON: &str = r#"
{
  "name": "reel_name",
  "frames": [ "frame1.fr.json", "frame2.fr.json"],
  "cut": {"KEY": "value"}
}
    "#;

    test_ser_de!(
        vframe,
        VirtualReel {
            name:   "reel_name".into(),
            frames: frames!(["frame1.fr.json", "frame2.fr.json"]),
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
            name:   "reel_name".into(),
            frames: frames!({"1" => PathBuf::from("other_reel.01s.name.fr.json")}),
            cut:    VirtualCut::MergeCuts(vec!["other_reel.cut.json".into()]),
        },
        PATH_VREEL_JSON
    );
}
