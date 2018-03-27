use chrono::prelude::*;
use std::fmt;
use std::collections::HashMap;
use serde;
use serde_qs as qs;

#[derive(Debug,Eq,PartialEq,Serialize,Deserialize)]
pub enum HttpMethod {
    HEAD,
    GET,
    POST,
    PUT,
    OPTIONS,
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

#[derive(Serialize,Deserialize)]
pub struct HttpEvent {
    pub body: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    #[serde(rename="httpMethod")]
    pub http_method: HttpMethod,
    #[serde(rename="isBase64Encoded")]
    pub is_base64_encoded: bool,
    pub path: String,
    #[serde(rename="pathParameters")]
    pub path_parameters: Option<HashMap<String, String>>,
    #[serde(rename="queryStringParameters")]
    pub query_string_parameters: Option<HashMap<String, String>>,
    pub resource: String,
    #[serde(rename="requestContext")]
    pub request_context: HttpEventRequestContext,
    #[serde(rename="stageVariables")]
    pub stage_variables: Option<HashMap<String, String>>,
}

impl HttpEvent {

    pub fn get_header(&self, key: &str) -> Option<&str> {
        match self.headers {
            Some(ref h) => h.get(key).map(|s| s.as_str()),
            None => None,
        }
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

#[derive(Serialize,Deserialize)]
pub struct HttpEventRequestContext {
    #[serde(rename="accountId")]
    pub account_id: String,
    #[serde(rename="apiId")]
    pub api_id: String,
    #[serde(rename="httpMethod")]
    pub http_method: HttpMethod,
    pub identity: HashMap<String, Option<String>>,
    pub path: String,
    pub protocol: Option<String>,
    #[serde(rename="requestId")]
    pub request_id: String,
    #[serde(rename="requestTime")]
    pub request_time: Option<String>,
    #[serde(rename="requestTimeEpoch")]
    pub request_time_epoch: Option<u64>,
    #[serde(rename="resourceId")]
    pub resource_id: String,
    #[serde(rename="resourcePath")]
    pub resource_path: String,
    pub stage: String,
}

impl HttpEventRequestContext {

    pub fn time(&self) -> Option<DateTime<Utc>> {
        // Utc::datetime_from_str(&self.request_time, "").ok()e
        None
    }
}

serde_aux_enum_number_declare!(HttpStatus {
    OK = 200,

    MovedPermanently = 301,
    Found = 302,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    Gone = 410,

    InternalServerError = 500,
    BadGateway = 502,
});

#[derive(Serialize,Deserialize)]
pub struct HttpResponse {
    #[serde(rename="statusCode")]
    pub status: HttpStatus,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    #[serde(rename="isBase64Encoded",default)]
    pub is_base64: bool,
}

impl HttpResponse {

    pub fn empty(status: HttpStatus) -> Self {
        HttpResponse {
            status: status,
            headers: HashMap::new(),
            body: None,
            is_base64: false,
        }
    }

    pub fn with_body<S: Into<String>>(status: HttpStatus, body: S) -> Self {
        HttpResponse {
            status: status,
            headers: HashMap::new(),
            body: Some(body.into()),
            is_base64: false,
        }
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(String::from(key), String::from(value));
    }

    pub fn success(&self) -> bool {
        return self.status >= HttpStatus::OK && self.status < HttpStatus::BadRequest
    }
}
