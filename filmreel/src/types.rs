use jql;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// https://github.com/Bestowinc/filmReel/blob/supra_dump/frame.md#frame-nomenclature
#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Protocol {
    GRPC,
    HTTP,
}

// /// https://github.com/Bestowinc/filmReel/blob/supra_dump/frame.md#frame
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Frame<'a> {
    protocol: Protocol,
    #[serde(borrow)]
    cut:      InstructionSet<'a>,
    request:  Request,
    response: Response,
}

// /// https://github.com/Bestowinc/filmReel/blob/supra_dump/frame.md#request
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Request {
    body: Value,
    #[serde(flatten)]
    etc:  Value,
    uri:  String,
}

/// https://github.com/Bestowinc/filmReel/blob/supra_dump/frame.md#cut-instruction-set
// This contains read and write instructions for the cut register, struct should be immutable after
// creation
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
struct InstructionSet<'a> {
    #[serde(alias = "from", borrow)]
    reads:  HashSet<&'a str>,
    #[serde(alias = "to", borrow)]
    writes: HashMap<&'a str, &'a str>,
}

/// https://github.com/Bestowinc/filmReel/blob/supra_dump/frame.md#request
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Response {
    body:   Value,
    #[serde(flatten)]
    etc:    Value,
    status: u32,
}

// to macro creates a write instuction HashMap
macro_rules! to {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map: HashMap<&str, &str> = std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

// from macro creates a read instuction HashSet
macro_rules! from {
    ($( $cut_var: expr ),*) => {{
         let mut set:HashSet<&str> = std::collections::HashSet::new();
         $( set.insert($cut_var); )*
         set
    }}
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const PROTOCOL_GRPC: &str = r#""GRPC""#;
    const PROTOCOL_HTTP: &str = r#""HTTP""#;
    #[test]
    fn protocol() {
        let grpc_json: Protocol = serde_json::from_str(PROTOCOL_GRPC).unwrap();
        let grpc: Protocol = Protocol::GRPC;
        assert_eq!(grpc_json, grpc);

        let http_json: Protocol = serde_json::from_str(PROTOCOL_HTTP).unwrap();
        let http: Protocol = Protocol::HTTP;
        assert_eq!(http_json, http);
    }

    const REQUEST: &str = r#"
{
  "body": {
    "email": "new_user@humanmail.com"
  },
  "uri": "user_api.User/CreateUser"
}
"#;
    #[test]
    fn request() {
        let json: Request = serde_json::from_str(REQUEST).unwrap();
        let request: Request = Request {
            body: json!({"email": "new_user@humanmail.com"}),
            etc:  json!({}),
            uri:  String::from("user_api.User/CreateUser"),
        };
        assert_eq!(json, request);
    }

    const ETC_REQUEST: &str = r#"
{
  "header": {
    "Authorization": "${USER_TOKEN}"
  },
  "id" : "007",
  "body": {},
  "uri": "POST /logout/${USER_ID}"
}
"#;
    #[test]
    fn request_etc() {
        let etc_json: Request = serde_json::from_str(ETC_REQUEST).unwrap();
        let etc_request: Request = Request {
            body: json!({}),
            etc:  json!({"header": { "Authorization": "${USER_TOKEN}" }, "id": "007"}),
            uri:  String::from("POST /logout/${USER_ID}"),
        };
        assert_eq!(etc_json, etc_request);
    }
    const RESPONSE: &str = r#"
{
  "body": "created user: ${USER_ID}",
  "status": 0
}
"#;
    #[test]
    fn response() {
        let json: Response = serde_json::from_str(RESPONSE).unwrap();
        let response: Response = Response {
            body:   json!("created user: ${USER_ID}"),
            etc:    json!({}),
            status: 0,
        };
        assert_eq!(json, response);
    }

    const ETC_RESPONSE: &str = r#"
{
  "body": "created user: ${USER_ID}",
  "user_level": "admin",
  "status": 0
}
"#;
    #[test]
    fn response_etc() {
        let etc_json: Response = serde_json::from_str(ETC_RESPONSE).unwrap();
        let etc_response: Response = Response {
            body:   json!("created user: ${USER_ID}"),
            etc:    json!({"user_level": "admin"}),
            status: 0,
        };
        assert_eq!(etc_json, etc_response);
    }

    const INSTRUCTION_SET: &str = r#"
{
  "from": [
    "USER_ID",
    "USER_TOKEN"
  ],
  "to": {
    "SESSION_ID": ".response.body.session_id",
    "DATETIME": ".response.body.timestamp"
  }
}
"#;
    #[test]
    fn instruction_set() {
        let json: InstructionSet = serde_json::from_str(INSTRUCTION_SET).unwrap();

        let writes: HashMap<&str, &str> = to![
            "SESSION_ID" => ".response.body.session_id",
            "DATETIME" => ".response.body.timestamp"];
        let reads: HashSet<&str> = from!["USER_ID", "USER_TOKEN"];

        let instructions = InstructionSet { writes, reads };
        assert_eq!(json, instructions);
    }
    const FRAME: &str = r#"
{
  "protocol": "HTTP",
  "cut": {
    "from": [
      "USER_ID",
      "USER_TOKEN"
    ],
    "to": {
      "SESSION_ID": ".response.body.session_id",
      "DATETIME": ".response.body.timestamp"
    }
  },
  "request": {
    "header": {
      "Authorization": "${USER_TOKEN}"
    },
    "body": {},
    "uri": "POST /logout/${USER_ID}"
  },
  "response": {
    "body": {
      "message": "User ${USER_ID} logged out",
      "session_id": "${SESSION_ID}",
      "timestamp": "${DATETIME}"
    },
    "status": 200
  }
}
"#;
    #[test]
    fn frame() {
        let json: Frame = serde_json::from_str(FRAME).unwrap();
        let frame = Frame {
            protocol: Protocol::HTTP,
            cut:      InstructionSet {
                reads:  from!["USER_ID", "USER_TOKEN"],
                writes: to!["SESSION_ID" => ".response.body.session_id", "DATETIME" => ".response.body.timestamp"],
            },
            request:  Request {
                body: json!({}),
                etc:  json!({"header": { "Authorization": "${USER_TOKEN}"}}),
                uri:  String::from("POST /logout/${USER_ID}"),
            },

            response: Response {
                body:   json!({
                  "message": "User ${USER_ID} logged out",
                  "session_id": "${SESSION_ID}",
                  "timestamp": "${DATETIME}"
                }),
                etc:    json!({}),
                status: 200,
            },
        };
        assert_eq!(json, frame, "\njson -> frame");

        // assert_eq!( // TODO macro
        //     serde_json::to_string_pretty(&frame).unwrap(),
        //     FRAME,
        //     "\nframe -> json"
        // );
    }
}
