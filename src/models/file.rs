use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Queryable, Deserialize, Serialize)]
pub struct File {
    pub id: i64,
    pub owner: i64,
    pub name: String,
    pub size: i32,
    pub path: String,
    pub file_type: i32,
    pub create_at: SystemTime,
    pub meta: String,
}

pub enum FileType {
    File,
    Dir,
}

impl File {
    pub fn new_file(owner: i64, name: String, path: String, size: i32) -> File {
        return File {
            id: 0,
            owner,
            name,
            size,
            path,
            file_type: FileType::File as i32,
            create_at: SystemTime::now(),
            meta: "".to_string(),
        };
    }

    pub fn new_dir(owner: i64, name: String, path: String) -> File {
        return File {
            id: 0,
            owner,
            name,
            size: 0,
            path,
            file_type: FileType::Dir as i32,
            create_at: SystemTime::now(),
            meta: "".to_string(),
        };
    }
}
