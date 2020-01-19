#[cfg(test)]
#[macro_use]
use serde_json::*;

/// test_ser_de tests the serialization and deserialization of frame structs
///
/// ```edition2018
///  test_ser_de!(
///      protocol_grpc_ser,  // serialization test name
///      protocol_grpc_de,   // deserialization test name
///      Protocol,           // struct type
///      Protocol::GRPC,     // struct
///      PROTOCOL_GRPC_JSON  // json format
/// );
/// ```
#[macro_export]
macro_rules! test_ser_de {
    ($ser:ident, $de:ident, $type:ty, $struct:expr, $str_json:expr) => {
        #[test]
        fn $ser() {
            let str_val: serde_json::Value = serde_json::from_str($str_json).unwrap();
            let actual = serde_json::value::to_value(&$struct).unwrap();
            assert_eq!(str_val, actual);
        }
        #[test]
        fn $de() {
            let actual: $type = serde_json::from_str($str_json).unwrap();
            assert_eq!(&$struct, &actual);
        }
    };
}
