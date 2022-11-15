use std::fmt::{write, Display, Formatter};

use actix_web::body::{BoxBody, MessageBody};
use actix_web::http::header::TryIntoHeaderPair;
use actix_web::http::StatusCode;
use actix_web::{
    http, http::header::ContentType, CustomizeResponder, HttpRequest, HttpResponse,
    HttpResponseBuilder, Responder, ResponseError,
};
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

pub type ServiceResult = Result<ServiceResponse, ServiceResponse>;

const INTERNAL_ERR_CODE_OFFSET: i32 = 10000;

#[derive(Serialize, Debug)]
pub struct ServiceResponse {
    #[serde(skip_serializing)]
    status_code: http::StatusCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    err_code: Option<ServiceError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    err_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
}

#[derive(Debug, Copy, Clone)]
pub enum ServiceError {
    ErrIllegalPathParam,
    ErrIllegalQueryParam,
    ErrIllegalUrlParam,
    ErrIllegalFormatData,
    ErrIllegalBodyData,
    ErrParseJson,
    ErrSerializeJson,
    ErrDeserializeJson,
    ErrDbPool,
    ErrMutexLock,
    ErrDbRecordExist,
    ErrDbRecordNotExist,
    ErrDbInsertFailed,
    ErrDbUpdateFailed,
    ErrDbDeleteFailed,
    ErrDbQueryFailed,
    ErrReadFileFailed,
    ErrWriteFileFailed,
    ErrDeleteFileFailed,
}

impl Display for ServiceResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = serde_json::to_string(self).unwrap_or(String::default());
        write!(f, "{}", str)
    }
}

impl ServiceResponse {
    fn new(
        status_code: http::StatusCode,
        err_code: Option<ServiceError>,
        err_msg: Option<String>,
        content: Option<impl Serialize>,
    ) -> Self {
        let result = if content.is_none() {
            None
        } else {
            Some(serde_json::json!(content))
        };
        ServiceResponse {
            status_code,
            err_code,
            err_msg,
            result,
        }
    }

    pub fn ok(value: impl Serialize) -> ServiceResponse {
        ServiceResponse::new(StatusCode::OK, None, None, Some(value))
    }

    pub fn err(
        code: http::StatusCode,
        err_code: ServiceError,
        msg: &'static str,
    ) -> ServiceResponse {
        let content: Option<&str> = None;
        ServiceResponse::new(code, Some(err_code), Some(msg.to_string()), content)
    }

    pub fn reply_err(
        code: http::StatusCode,
        err_code: ServiceError,
        msg: &'static str,
    ) -> Result<ServiceResponse, ServiceResponse> {
        let resp = ServiceResponse::err(code, err_code, msg);
        Err(resp)
    }

    fn reply(code: http::StatusCode, value: impl Serialize) -> Result<Self, ServiceResponse> {
        let content = serde_json::json!(value);
        let mut resp = ServiceResponse::new(code, None, None, Some(content));
        Ok(resp)
    }

    pub fn reply_ok(value: impl Serialize) -> ServiceResult {
        ServiceResponse::reply(http::StatusCode::OK, value)
    }

    pub fn reply_bad_request(err_code: ServiceError, msg: &'static str) -> ServiceResult {
        ServiceResponse::reply_err(http::StatusCode::BAD_REQUEST, err_code, msg)
    }

    pub fn reply_intern_err(err_code: ServiceError, msg: &'static str) -> ServiceResult {
        ServiceResponse::reply_err(http::StatusCode::INTERNAL_SERVER_ERROR, err_code, msg)
    }
}

pub fn DbPoolErr(e: r2d2::Error) -> ServiceResponse {
    ServiceResponse::err(
        http::StatusCode::INTERNAL_SERVER_ERROR,
        ServiceError::ErrDbPool,
        "internal server error",
    )
}

impl Responder for ServiceResponse {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<BoxBody> {
        let content = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(content)
    }
}

impl ResponseError for ServiceResponse {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let content = serde_json::to_string(self).unwrap();
        HttpResponseBuilder::new(self.status_code())
            .content_type(ContentType::json())
            .body(content.boxed())
    }
}

impl serde::Serialize for ServiceError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as i32 + INTERNAL_ERR_CODE_OFFSET).serialize(serializer)
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (*self as i32 + INTERNAL_ERR_CODE_OFFSET))
    }
}
