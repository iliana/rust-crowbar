use super::*;

use serde_json;

#[test]
fn test_auth_payload() {
    let event: AuthEvent = serde_json::from_str(include_str!("fixtures/authorize.json"))
        .expect("unable to parse json");

    assert_eq!("GET", event.http_method);
    assert_eq!("/", event.path);
    assert_eq!("/", event.resource);
    assert_eq!("arn:aws:execute-api:us-east-1:961179389914:smddwihyy9/null/GET/",
        event.method_arn);
    assert_eq!(AuthEventType::Request, event.event_type);
}

#[test]
fn test_https_payload() {
    let event: HttpEvent = serde_json::from_str(include_str!("fixtures/request.json"))
        .expect("unable to parse json");

    // fields themselves
    assert_eq!(None, event.body);
    assert_eq!(HttpMethod::GET , event.http_method);
    assert_eq!(false, event.is_base64_encoded);
    assert_eq!("/echo.json", event.path);
    assert_eq!("/echo.json", event.resource);

    // subfields
    assert_eq!("961179389914", event.request_context.account_id);
    assert_eq!("smddwihyy9", event.request_context.api_id);
    assert_eq!(HttpMethod::GET, event.request_context.http_method);
    assert_eq!("/echo.json", event.request_context.path);
    assert_eq!("HTTP/1.1", event.request_context.protocol.unwrap());
    assert_eq!("07535b6a-18e9-11e8-bcc6-397efc46efd2", event.request_context.request_id);
    assert_eq!("23/Feb/2018:22:29:36 +0000", event.request_context.request_time.unwrap());
    assert_eq!(1519424976415, event.request_context.request_time_epoch.unwrap());
    assert_eq!("lig9h2", event.request_context.resource_id);
    assert_eq!("/echo.json", event.request_context.resource_path);
    assert_eq!("production", event.request_context.stage);
}
