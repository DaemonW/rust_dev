use actix_web::{
    delete, get, http, http::header::ContentType, post, put, web, HttpRequest, HttpResponse,
    Responder, ResponseError,
};
use diesel::prelude::*;
use diesel::query_builder::AsChangeset;
use serde::Deserialize;
use tracing::error;

use crate::models::db::{DbPool, SqlErr, SqlErrKind};
use crate::models::response::{DbPoolErr, ServiceError, ServiceResponse, ServiceResult};
use crate::models::schema;
use crate::models::user::*;
use crate::utils::pattern;

const INFO_FIELD: [&'static str; 4] = ["age", "sex", "address", "ip"];

#[derive(Deserialize)]
pub struct UserFilter {
    name: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    role: Option<i32>,
    status: Option<i32>,
}

#[get("/users")]
pub async fn list_user(pool: web::Data<DbPool>, params: web::Query<UserFilter>) -> ServiceResult {
    let conn = &mut pool.get().map_err(DbPoolErr)?;

    let params = params.into_inner();
    let mut query = schema::users::table.into_boxed();
    if let Some(name) = params.name {
        if !name.is_empty() {
            query = query.filter(schema::users::username.eq(name));
        }
    }
    if let Some(phone) = params.phone {
        if !phone.is_empty() {
            query = query.filter(schema::users::phone.eq(phone));
        }
    }
    if let Some(email) = params.email {
        if !email.is_empty() {
            query = query.filter(schema::users::email.eq(email));
        }
    }
    if let Some(role) = params.role {
        query = query.filter(schema::users::role.eq(role));
    }
    if let Some(status) = params.status {
        query = query.filter(schema::users::status.eq(status));
    }

    match query.load::<User>(conn) {
        Ok(ret) => ServiceResponse::reply_ok(ret),
        Err(e) => {
            error!("{}", e);
            ServiceResponse::reply_intern_err(
                ServiceError::ErrDbQueryFailed,
                "query users info failed",
            )
        }
    }
}

#[get("/user/{user_id}")]
pub async fn get_user(pool: web::Data<DbPool>, path: web::Path<String>) -> ServiceResult {
    let conn = &mut pool.get().map_err(DbPoolErr)?;
    let user_id: i64 = path.into_inner().parse().unwrap_or(-1);
    let query = schema::users::table;
    let result = if user_id < 0 {
        query.load::<User>(conn)
    } else {
        query
            .filter(schema::users::id.eq(user_id))
            .load::<User>(conn)
    };
    match result {
        Ok(ret) => ServiceResponse::reply_ok(ret),
        Err(e) => {
            error!("{}", e);
            ServiceResponse::reply_intern_err(
                ServiceError::ErrDbQueryFailed,
                "query user info failed",
            )
        }
    }
}

#[delete("/user/{user_id}")]
pub async fn delete_user(pool: web::Data<DbPool>, _id: web::Path<String>) -> ServiceResult {
    let user_id: i64 = _id.into_inner().parse().unwrap_or(-1);
    if user_id < 0 {
        ServiceResponse::reply_bad_request(ServiceError::ErrIllegalPathParam, "illegal user id")
    } else {
        let conn = &mut pool.get().map_err(DbPoolErr)?;
        let result =
            diesel::delete(schema::users::dsl::users.filter(schema::users::id.eq(user_id)))
                .execute(conn);
        match result {
            Ok(_) => ServiceResponse::reply_ok("delete user success"),
            Err(e) => {
                error!("{}", e);
                ServiceResponse::reply_intern_err(
                    ServiceError::ErrDbDeleteFailed,
                    "delete user failed",
                )
            }
        }
    }
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = schema::users)]
pub struct UpdateUserInfo {
    password: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    info: Option<serde_json::Value>,
}

#[put("user/{user_id}")]
pub async fn update_user_info(
    _pool: web::Data<DbPool>,
    _id: web::Path<String>,
    _params: web::Query<UpdateUserInfo>,
) -> ServiceResult {
    let id: i64 = _id.into_inner().parse().unwrap_or(-1);
    if id < 0 {
        return ServiceResponse::reply_bad_request(
            ServiceError::ErrIllegalPathParam,
            "illegal user id",
        );
    }
    let params = _params.into_inner();
    if params.password.is_none()
        && params.phone.is_none()
        && params.email.is_none()
        && params.info.is_none()
    {
        return ServiceResponse::reply_bad_request(
            ServiceError::ErrIllegalQueryParam,
            "illegal update params",
        );
    }
    if let Err(e) = check_update_params(&params) {
        return ServiceResponse::reply_bad_request(ServiceError::ErrIllegalQueryParam, e);
    }
    let conn = &mut _pool.get().map_err(DbPoolErr)?;
    let result = diesel::update(schema::users::dsl::users.filter(schema::users::id.eq(id)))
        .set(&params)
        .execute(conn);
    return match result {
        Ok(n) => {
            if n > 0 {
                ServiceResponse::reply_ok("update user success")
            } else {
                ServiceResponse::reply_bad_request(
                    ServiceError::ErrDbRecordNotExist,
                    "update user failed",
                )
            }
        }
        Err(e) => {
            error!("{}", e);
            ServiceResponse::reply_intern_err(ServiceError::ErrDbUpdateFailed, "update user failed")
        }
    };
}

fn check_update_params<'a>(info: &'a UpdateUserInfo) -> Result<(), &'static str> {
    if let Some(password) = info.password.as_ref() {
        pattern::password(password)?
    }
    if let Some(phone) = info.phone.as_ref() {
        pattern::phone(phone)?
    }
    if let Some(email) = info.email.as_ref() {
        pattern::password(email)?
    }
    Ok(())
}

#[derive(Deserialize)]
pub struct RegisterUserInfo {
    username: Option<String>,
    password: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    info: Option<serde_json::Value>,
}

#[post("/users")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    params: web::Json<RegisterUserInfo>,
) -> ServiceResult {
    let conn = &mut pool.get().map_err(DbPoolErr)?;
    let register = params.into_inner();
    if let Err(e) = check_user_param(&register) {
        return ServiceResponse::reply_bad_request(ServiceError::ErrIllegalBodyData, e);
    }
    let empty_info = serde_json::Value::String("".to_string());
    let result = diesel::insert_into(schema::users::table)
        .values((
            schema::users::username.eq(register.username.unwrap_or("".to_string())),
            schema::users::password.eq(register.password.unwrap_or("".to_string())),
            schema::users::phone.eq(register.phone.unwrap_or("".to_string())),
            schema::users::email.eq(register.email.unwrap_or("".to_string())),
            schema::users::role.eq(Role::USER as i32),
            schema::users::status.eq(Status::FREEZE as i32),
            schema::users::info.eq(register.info.unwrap_or(empty_info)),
        ))
        .get_result::<User>(conn);
    let resp = match result {
        Ok(user) => ServiceResponse::reply_ok(user),
        Err(e) => {
            if let SqlErr::DatabaseError(SqlErrKind::UniqueViolation, _) = e {
                ServiceResponse::reply_bad_request(
                    ServiceError::ErrDbRecordExist,
                    "user already exists",
                )
            } else {
                error!("{}", e);
                ServiceResponse::reply_intern_err(
                    ServiceError::ErrDbInsertFailed,
                    "register user failed",
                )
            }
        }
    };
    return resp;
}

fn check_user_param<'a>(info: &'a RegisterUserInfo) -> Result<(), &'static str> {
    if let Some(username) = info.username.as_ref() {
        pattern::username(username)?;
    } else {
        return Err("username param is missing");
    }
    if let Some(password) = info.password.as_ref() {
        pattern::password(password)?;
    } else {
        return Err("password param is missing");
    }
    if let Some(phone) = &info.phone {
        pattern::phone(phone)?;
    }
    if let Some(email) = &info.email {
        pattern::email(email)?;
    }
    Ok(())
}
