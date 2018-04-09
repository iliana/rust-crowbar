/// Data-types for Amazon API Gateway.

mod auth;
#[cfg(test)]
mod tests;

pub use self::auth::AuthEvent;
pub use self::auth::AuthEventType;
pub use self::auth::AuthEffect;

use super::*;

use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;

use std::fmt;

use serde_qs as qs;

/// A generic API Gateway event, either an `AuthEvent` or a `HttpEvent`.
pub enum Event {
    /// An authorization request from API Gateway requesting access to a given resource.
    Authorize(AuthEvent),
    /// An API Gateway Lambda Proxy request.
    Request(HttpEvent),
}

/// An enumeration over HTTP methods.
#[derive(Debug,Eq,PartialEq,Serialize,Deserialize)]
pub enum HttpMethod {
    /// A HTTP HEAD request.
    HEAD,
    /// A HTTP GET request.
    GET,
    /// A HTTP POST request.
    POST,
    /// A HTTP PUT request.
    PUT,
    /// A HTTP OPTIONS request.
    OPTIONS,
    /// A HTTP DELETE request.
    DELETE,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            HttpMethod::HEAD => "HEAD",
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::DELETE => "DELETE"
        })
    }
}

/// An event type representing an API Gateway Proxy Request as sent to a Lambda function.
#[derive(Serialize,Deserialize)]
pub struct HttpEvent {
    /// The optional body of the request.
    /// If `is_base64_encoded` is `true`, this will be a base-64 encoded binary payload.
    pub body: Option<String>,
    /// A map of HTTP headers present with the request.
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
    /// The HTTP method used in the request.
    #[serde(rename="httpMethod")]
    pub http_method: HttpMethod,
    /// Whether the `body` is plaintext or base-64 encoded binary data.
    #[serde(rename="isBase64Encoded")]
    pub is_base64_encoded: bool,
    /// The request path.
    pub path: String,
    /// A map of path parameters defined by the API Gateway Resource.
    #[serde(default,rename="pathParameters")]
    pub path_parameters: BTreeMap<String, String>,
    /// A map of query string parameters extracted from the request.
    #[serde(default,rename="queryStringParameters")]
    pub query_string_parameters: BTreeMap<String, String>,
    /// The API Gateway Resource id for the current resource being requested.
    pub resource: String,
    /// The HTTP request context for this request.
    #[serde(rename="requestContext")]
    pub request_context: HttpEventRequestContext,
    /// A map of API Gateway Stage Variables for this request.
    #[serde(default,rename="stageVariables")]
    pub stage_variables: BTreeMap<String, String>,
}

impl HttpEvent {

    /// Fetch a header from this event, if present.
    pub fn header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|s| s.as_str())
    }
}

impl fmt::Display for HttpEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{protocol} {method} {path}{querystring} (body? {body})",
            protocol = match self.request_context.protocol {
                Some(ref s) => s.as_str(),
                None => "(unknown)"
            },
            method = self.http_method,
            path = self.path,
            querystring = qs::to_string(&self.query_string_parameters).unwrap_or(String::new()),
            body = match self.body {
                Some(_) => true,
                None => false
            }
        )
    }
}

/// A HTTP event request context containing metadata about the request.
#[derive(Serialize,Deserialize)]
pub struct HttpEventRequestContext {
    /// The AWS account id of the current AWS account.
    #[serde(rename="accountId")]
    pub account_id: String,
    /// The API Gateway Rest API id for this request.
    #[serde(rename="apiId")]
    pub api_id: String,
    /// The HTTP method of the request.
    #[serde(rename="httpMethod")]
    pub http_method: HttpMethod,
    /// Identity information relevant to the currently executing Lambda function.
    /// Some keys may be null.
    pub identity: BTreeMap<String, Option<String>>,
    /// The HTTP path of the request.
    pub path: String,
    /// The protocol used in this request.
    pub protocol: Option<String>,
    /// The AWS request id.
    #[serde(rename="requestId")]
    pub request_id: String,
    /// The request timestamp.
    #[serde(rename="requestTime")]
    pub request_time: Option<String>,
    /// The request timestamp as a `u64`.
    #[serde(rename="requestTimeEpoch")]
    pub request_time_epoch: Option<u64>,
    /// The API Gateway Resource id.
    #[serde(rename="resourceId")]
    pub resource_id: String,
    /// The path to this API Gateway Resource.
    #[serde(rename="resourcePath")]
    pub resource_path: String,
    /// The API Gateway Stage name.
    pub stage: String,
}

impl HttpEventRequestContext {

    /// Returns the request time as a `chrono::DateTime<Utc>`.
    ///
    /// This value is parsed from `request_time_epoch`, which is the number of milliseconds since the
    /// Unix epoch.
    pub fn time(&self) -> Option<DateTime<Utc>> {
        match self.request_time_epoch {
            Some(ts) => Some(Utc.timestamp((ts / 1_000) as i64, ((ts % 1000) * 1_000_000) as u32)),
            None => None,
        }
    }
}

/// An enumeration over standard HTTP status codes.
serde_aux_enum_number_declare!(
HttpStatus {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,

    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    IMUsed = 226,

    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    SwitchProxy = 306,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    URITooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,

    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequest = 511,
});

/// A HTTP response to be sent back to a client.
#[derive(Serialize,Deserialize)]
pub struct HttpResponse {
    #[serde(rename="statusCode")]
    pub status: HttpStatus,
    pub headers: BTreeMap<String, String>,
    pub body: Option<String>,
    #[serde(default)]
    pub is_base64_encoded: bool,
}

impl HttpResponse {

    pub fn empty(status: HttpStatus) -> Self {
        HttpResponse {
            status: status,
            headers: BTreeMap::new(),
            body: None,
            is_base64_encoded: false,
        }
    }

    pub fn with_body<S: Into<String>>(status: HttpStatus, body: S) -> Self {
        HttpResponse {
            status: status,
            headers: BTreeMap::new(),
            body: Some(body.into()),
            is_base64_encoded: false,
        }

    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(String::from(key), String::from(value));
    }

    pub fn success(&self) -> bool {
        return self.status >= HttpStatus::Ok && self.status < HttpStatus::BadRequest
    }
}
