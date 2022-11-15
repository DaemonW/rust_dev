use actix_web::body::{BoxBody, MessageBody};
use actix_web::http::StatusCode;
use actix_web::{
    http, http::header::ContentType, HttpRequest, HttpResponse, Responder, ResponseError,
};
use serde::{Serialize, Serializer};
use serde_json::Value;
use std::fmt::{Display, Formatter};

pub type ServiceResult = Result<ServiceResponse, ServiceResponse>;

const INTERNAL_ERR_CODE_OFFSET: i32 = 10000;

#[derive(Serialize, Debug)]
pub struct ServiceResponse {
    #[serde(skip_serializing)]
    status_code: u16,
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
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl ServiceResponse {
    pub fn new(code: http::StatusCode) -> Self {
        ServiceResponse {
            status_code: code.as_u16(),
            err_code: None,
            err_msg: None,
            result: None,
        }
    }

    pub fn reply_err(
        code: http::StatusCode,
        err_code: ServiceError,
        msg: &'static str,
    ) -> Result<ServiceResponse, ServiceResponse> {
        let mut resp = ServiceResponse::new(code);
        resp.err_msg = Some(String::from(msg));
        resp.err_code = Some(err_code);
        Err(resp)
    }

    fn reply(code: http::StatusCode, value: impl Serialize) -> Result<Self, ServiceResponse> {
        let mut resp = ServiceResponse::new(code);
        resp.result = Some(serde_json::json!(value));
        Ok(resp)
    }

    pub fn ok(value: impl Serialize) -> ServiceResult {
        ServiceResponse::reply(http::StatusCode::OK, value)
    }

    pub fn bad_request(err_code: ServiceError, msg: &'static str) -> ServiceResult {
        ServiceResponse::reply_err(http::StatusCode::BAD_REQUEST, err_code, msg)
    }

    pub fn intern_err(err_code: ServiceError, msg: &'static str) -> ServiceResult {
        ServiceResponse::reply_err(http::StatusCode::INTERNAL_SERVER_ERROR, err_code, msg)
    }
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
        http::StatusCode::from_u16(self.status_code).unwrap()
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let content = serde_json::to_string(self).unwrap();
        HttpResponse::with_body(self.status_code(), content.boxed())
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
