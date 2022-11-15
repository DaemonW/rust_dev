use crate::models::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub phone: String,
    pub email: String,
    pub role: i32,
    pub status: i32,
    pub info: serde_json::Value,
}

pub enum Status {
    FREEZE,
    NORMAL,
}

pub enum Role {
    USER,
    ADMIN,
}

impl User {
    pub fn new(name: String, password: String) -> User {
        return User {
            id: 0,
            username: name,
            password,
            phone: "".to_string(),
            email: "".to_string(),
            role: Role::USER as i32,
            status: Status::FREEZE as i32,
            info: serde_json::from_str("").unwrap(),
        };
    }
}
