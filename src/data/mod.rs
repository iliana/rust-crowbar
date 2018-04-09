/// Data types for Amazon API Gateway.
///
/// These types are defined [in the Amazon API Gateway docs][apigateway-data-types].
///
///  [apigateway-data-types]: https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-lambda-proxy-integrations.html
pub mod apigateway;

use serde;
use serde::de::Deserializer;

use std::collections::BTreeMap;

// fn deserialize_empty<T>() -> T where T: Default {
//     T::default()
// }

// fn deserialize_with_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
//         where D: Deserializer<'de> {
//     let result = deserializer.deserialize_any();
//
//     result
// }
