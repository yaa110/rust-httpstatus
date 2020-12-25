// In the name of Allah

//! List of HTTP response status codes

#![crate_name = "httpstatus"]

use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt::{Display, Error, Formatter};

/// Represents an HTTP status code
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Serialize, Deserialize)]
pub enum StatusCode {
    /// 100 Continue (RFC 7231)
    Continue,

    /// 101 Switching Protocols (RFC 7231)
    SwitchingProtocols,

    /// 102 Processing (RFC 2518)
    Processing,

    /// 103 Early Hints (RFC 8297)
    EarlyHints,

    /// 200 OK (RFC 7231)
    Ok,

    /// 201 Created (RFC 7231)
    Created,

    /// 202 Accepted (RFC 7231)
    Accepted,

    /// 203 Non-Authoritative Information (RFC 7231)
    NonAuthoritativeInformation,

    /// 204 No Content (RFC 7231)
    NoContent,

    /// 205 Reset Content (RFC 7231)
    ResetContent,

    /// 206 Partial Content (RFC 7233)
    PartialContent,

    /// 207 Multi-Status (RFC 4918)
    MultiStatus,

    /// 208 Already Reported (RFC 5842)
    AlreadyReported,

    /// 226 IM Used (RFC 3229)
    IMUsed,

    /// 300 Multiple Choices (RFC 7231)
    MultipleChoices,

    /// 301 Moved Permanently (RFC 7231)
    MovedPermanently,

    /// 302 Found (RFC 7231)
    Found,

    /// 303 See Other (RFC 7231)
    SeeOther,

    /// 304 Not Modified (RFC 7232)
    NotModified,

    /// 305 Use Proxy (RFC 7231)
    UseProxy,

    /// 306 Switch Proxy (RFC 7231)
    SwitchProxy,

    /// 307 Temporary Redirect (RFC 7231)
    TemporaryRedirect,

    /// 308 Permanent Redirect (RFC 7538)
    PermanentRedirect,

    /// 400 Bad Request (RFC 7231)
    BadRequest,

    /// 401 Unauthorized (RFC 7235)
    Unauthorized,

    /// 402 Payment Required (RFC 7231)
    PaymentRequired,

    /// 403 Forbidden (RFC 7231)
    Forbidden,

    /// 404 Not Found (RFC 7231)
    NotFound,

    /// 405 Method Not Allowed (RFC 7231)
    MethodNotAllowed,

    /// 406 Not Acceptable (RFC 7231)
    NotAcceptable,

    /// 407 Proxy Authentication Required (RFC 7235)
    ProxyAuthenticationRequired,

    /// 408 Request Timeout (RFC 7231)
    RequestTimeout,

    /// 409 Conflict (RFC 7231)
    Conflict,

    /// 410 Gone (RFC 7231)
    Gone,

    /// 411 Length Required (RFC 7231)
    LengthRequired,

    /// 412 Precondition Failed (RFC 7232)
    PreconditionFailed,

    /// 413 Payload Too Large (RFC 7231)
    PayloadTooLarge,

    /// 414 URI Too Long (RFC 7231)
    UriTooLong,

    /// 415 Unsupported Media Type (RFC 7231)
    UnsupportedMediaType,

    /// 416 Range Not Satisfiable (RFC 7233)
    RangeNotSatisfiable,

    /// 417 Expectation Failed (RFC 7231)
    ExpectationFailed,

    /// 418 I'm a teapot (RFC 2324)
    ImATeapot,

    /// 421 Misdirected Request (RFC 7540)
    MisdirectedRequest,

    /// 422 Unprocessable Entity (RFC 4918)
    UnprocessableEntity,

    /// 423 Locked (RFC 4918)
    Locked,

    /// 424 Failed Dependency (RFC 4918)
    FailedDependency,

    /// 426 Upgrade Required (RFC 7231)
    UpgradeRequired,

    /// 428 Precondition Required (RFC 6585)
    PreconditionRequired,

    /// 429 Too Many Requests (RFC 6585)
    TooManyRequests,

    /// 431 Request Header Fields Too Large (RFC 6585)
    RequestHeaderFieldsTooLarge,

    /// 451 Unavailable For Legal Reasons (RFC 7725)
    UnavailableForLegalReasons,

    /// 500 Internal Server Error (RFC 7231)
    InternalServerError,

    /// 501 Not Implemented (RFC 7231)
    NotImplemented,

    /// 502 Bad Gateway (RFC 7231)
    BadGateway,

    /// 503 Service Unavailable (RFC 7231)
    ServiceUnavailable,

    /// 504 Gateway Timeout (RFC 7231)
    GatewayTimeout,

    /// 505 HTTP Version Not Supported (RFC 7231)
    HttpVersionNotSupported,

    /// 506 Variant Also Negotiates (RFC 2295)
    VariantAlsoNegotiates,

    /// 507 Insufficient Storage (RFC 4918)
    InsufficientStorage,

    /// 508 Loop Detected (RFC 5842)
    LoopDetected,

    /// 510 Not Extended (RFC 2774)
    NotExtended,

    /// 511 Network Authentication Required (RFC 6585)
    NetworkAuthenticationRequired,

    /// Unknown status code
    Unknown(u16),
}

/// Represents an HTTP status class
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Serialize, Deserialize)]
pub enum StatusClass {
    /// 1xx Informational
    Informational,

    /// 2xx Success
    Success,

    /// 3xx Redirection
    Redirection,

    /// 4xx Client errors
    ClientError,

    /// 5xx Server errors
    ServerError,

    /// Unknown status class
    Unknown,
}

impl StatusCode {
    /// Returns the numeric status code
    #[inline]
    pub fn as_u16(&self) -> u16 {
        match *self {
            StatusCode::Continue => 100,
            StatusCode::SwitchingProtocols => 101,
            StatusCode::Processing => 102,
            StatusCode::EarlyHints => 103,
            StatusCode::Ok => 200,
            StatusCode::Created => 201,
            StatusCode::Accepted => 202,
            StatusCode::NonAuthoritativeInformation => 203,
            StatusCode::NoContent => 204,
            StatusCode::ResetContent => 205,
            StatusCode::PartialContent => 206,
            StatusCode::MultiStatus => 207,
            StatusCode::AlreadyReported => 208,
            StatusCode::IMUsed => 226,
            StatusCode::MultipleChoices => 300,
            StatusCode::MovedPermanently => 301,
            StatusCode::Found => 302,
            StatusCode::SeeOther => 303,
            StatusCode::NotModified => 304,
            StatusCode::UseProxy => 305,
            StatusCode::SwitchProxy => 306,
            StatusCode::TemporaryRedirect => 307,
            StatusCode::PermanentRedirect => 308,
            StatusCode::BadRequest => 400,
            StatusCode::Unauthorized => 401,
            StatusCode::PaymentRequired => 402,
            StatusCode::Forbidden => 403,
            StatusCode::NotFound => 404,
            StatusCode::MethodNotAllowed => 405,
            StatusCode::NotAcceptable => 406,
            StatusCode::ProxyAuthenticationRequired => 407,
            StatusCode::RequestTimeout => 408,
            StatusCode::Conflict => 409,
            StatusCode::Gone => 410,
            StatusCode::LengthRequired => 411,
            StatusCode::PreconditionFailed => 412,
            StatusCode::PayloadTooLarge => 413,
            StatusCode::UriTooLong => 414,
            StatusCode::UnsupportedMediaType => 415,
            StatusCode::RangeNotSatisfiable => 416,
            StatusCode::ExpectationFailed => 417,
            StatusCode::ImATeapot => 418,
            StatusCode::MisdirectedRequest => 421,
            StatusCode::UnprocessableEntity => 422,
            StatusCode::Locked => 423,
            StatusCode::FailedDependency => 424,
            StatusCode::UpgradeRequired => 426,
            StatusCode::PreconditionRequired => 428,
            StatusCode::TooManyRequests => 429,
            StatusCode::RequestHeaderFieldsTooLarge => 431,
            StatusCode::UnavailableForLegalReasons => 451,
            StatusCode::InternalServerError => 500,
            StatusCode::NotImplemented => 501,
            StatusCode::BadGateway => 502,
            StatusCode::ServiceUnavailable => 503,
            StatusCode::GatewayTimeout => 504,
            StatusCode::HttpVersionNotSupported => 505,
            StatusCode::VariantAlsoNegotiates => 506,
            StatusCode::InsufficientStorage => 507,
            StatusCode::LoopDetected => 508,
            StatusCode::NotExtended => 510,
            StatusCode::NetworkAuthenticationRequired => 511,
            StatusCode::Unknown(code) => code,
        }
    }

    /// Returns the reason phrase of HTTP status code
    #[inline]
    pub fn reason_phrase(&self) -> &'static str {
        match *self {
            StatusCode::Continue => "Continue",
            StatusCode::SwitchingProtocols => "Switching Protocols",
            StatusCode::Processing => "Processing",
            StatusCode::EarlyHints => "Early Hints",
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::Accepted => "Accepted",
            StatusCode::NonAuthoritativeInformation => "Non-Authoritative Information",
            StatusCode::NoContent => "No Content",
            StatusCode::ResetContent => "Reset Content",
            StatusCode::PartialContent => "Partial Content",
            StatusCode::MultiStatus => "Multi-Status",
            StatusCode::AlreadyReported => "Already Reported",
            StatusCode::IMUsed => "IM Used",
            StatusCode::MultipleChoices => "Multiple Choices",
            StatusCode::MovedPermanently => "Moved Permanently",
            StatusCode::Found => "Found",
            StatusCode::SeeOther => "See Other",
            StatusCode::NotModified => "Not Modified",
            StatusCode::UseProxy => "Use Proxy",
            StatusCode::SwitchProxy => "Switch Proxy",
            StatusCode::TemporaryRedirect => "Temporary Redirect",
            StatusCode::PermanentRedirect => "Permanent Redirect",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::PaymentRequired => "Payment Required",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::NotAcceptable => "Not Acceptable",
            StatusCode::ProxyAuthenticationRequired => "Proxy Authentication Required",
            StatusCode::RequestTimeout => "Request Timeout",
            StatusCode::Conflict => "Conflict",
            StatusCode::Gone => "Gone",
            StatusCode::LengthRequired => "Length Required",
            StatusCode::PreconditionFailed => "Precondition Failed",
            StatusCode::PayloadTooLarge => "Payload Too Large",
            StatusCode::UriTooLong => "URI Too Long",
            StatusCode::UnsupportedMediaType => "Unsupported Media Type",
            StatusCode::RangeNotSatisfiable => "Range Not Satisfiable",
            StatusCode::ExpectationFailed => "Expectation Failed",
            StatusCode::ImATeapot => "I'm a teapot",
            StatusCode::MisdirectedRequest => "Misdirected Request",
            StatusCode::UnprocessableEntity => "Unprocessable Entity",
            StatusCode::Locked => "Locked",
            StatusCode::FailedDependency => "Failed Dependency",
            StatusCode::UpgradeRequired => "Upgrade Required",
            StatusCode::PreconditionRequired => "Precondition Required",
            StatusCode::TooManyRequests => "Too Many Requests",
            StatusCode::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            StatusCode::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::NotImplemented => "Not Implemented",
            StatusCode::BadGateway => "Bad Gateway",
            StatusCode::ServiceUnavailable => "Service Unavailable",
            StatusCode::GatewayTimeout => "Gateway Timeout",
            StatusCode::HttpVersionNotSupported => "Http Version Not Supported",
            StatusCode::VariantAlsoNegotiates => "Variant Also Negotiates",
            StatusCode::InsufficientStorage => "Insufficient Storage",
            StatusCode::LoopDetected => "Loop Detected",
            StatusCode::NotExtended => "Not Extended",
            StatusCode::NetworkAuthenticationRequired => "Network Authentication Required",
            StatusCode::Unknown(_) => "",
        }
    }

    /// Returns the status class of status code
    #[inline]
    pub fn class(&self) -> StatusClass {
        match self.as_u16() {
            100..=199 => StatusClass::Informational,
            200..=299 => StatusClass::Success,
            300..=399 => StatusClass::Redirection,
            400..=499 => StatusClass::ClientError,
            500..=599 => StatusClass::ServerError,
            _ => StatusClass::Unknown,
        }
    }
}

impl From<u16> for StatusCode {
    fn from(code: u16) -> Self {
        match code {
            100 => StatusCode::Continue,
            101 => StatusCode::SwitchingProtocols,
            102 => StatusCode::Processing,
            103 => StatusCode::EarlyHints,
            200 => StatusCode::Ok,
            201 => StatusCode::Created,
            202 => StatusCode::Accepted,
            203 => StatusCode::NonAuthoritativeInformation,
            204 => StatusCode::NoContent,
            205 => StatusCode::ResetContent,
            206 => StatusCode::PartialContent,
            207 => StatusCode::MultiStatus,
            208 => StatusCode::AlreadyReported,
            226 => StatusCode::IMUsed,
            300 => StatusCode::MultipleChoices,
            301 => StatusCode::MovedPermanently,
            302 => StatusCode::Found,
            303 => StatusCode::SeeOther,
            304 => StatusCode::NotModified,
            305 => StatusCode::UseProxy,
            306 => StatusCode::SwitchProxy,
            307 => StatusCode::TemporaryRedirect,
            308 => StatusCode::PermanentRedirect,
            400 => StatusCode::BadRequest,
            401 => StatusCode::Unauthorized,
            402 => StatusCode::PaymentRequired,
            403 => StatusCode::Forbidden,
            404 => StatusCode::NotFound,
            405 => StatusCode::MethodNotAllowed,
            406 => StatusCode::NotAcceptable,
            407 => StatusCode::ProxyAuthenticationRequired,
            408 => StatusCode::RequestTimeout,
            409 => StatusCode::Conflict,
            410 => StatusCode::Gone,
            411 => StatusCode::LengthRequired,
            412 => StatusCode::PreconditionFailed,
            413 => StatusCode::PayloadTooLarge,
            414 => StatusCode::UriTooLong,
            415 => StatusCode::UnsupportedMediaType,
            416 => StatusCode::RangeNotSatisfiable,
            417 => StatusCode::ExpectationFailed,
            418 => StatusCode::ImATeapot,
            421 => StatusCode::MisdirectedRequest,
            422 => StatusCode::UnprocessableEntity,
            423 => StatusCode::Locked,
            424 => StatusCode::FailedDependency,
            426 => StatusCode::UpgradeRequired,
            428 => StatusCode::PreconditionRequired,
            429 => StatusCode::TooManyRequests,
            431 => StatusCode::RequestHeaderFieldsTooLarge,
            451 => StatusCode::UnavailableForLegalReasons,
            500 => StatusCode::InternalServerError,
            501 => StatusCode::NotImplemented,
            502 => StatusCode::BadGateway,
            503 => StatusCode::ServiceUnavailable,
            504 => StatusCode::GatewayTimeout,
            505 => StatusCode::HttpVersionNotSupported,
            506 => StatusCode::VariantAlsoNegotiates,
            507 => StatusCode::InsufficientStorage,
            508 => StatusCode::LoopDetected,
            510 => StatusCode::NotExtended,
            511 => StatusCode::NetworkAuthenticationRequired,
            _ => StatusCode::Unknown(code),
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{}{}{}",
            self.as_u16(),
            if let StatusCode::Unknown(_) = *self {
                ""
            } else {
                " "
            },
            self.reason_phrase()
        )
    }
}
