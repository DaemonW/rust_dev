use crate::models::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Asset {
    id: i64,
    size: i32,
    hash: String,
    owner_ref: i32,
    create_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::assets)]
struct AssetRecord<'a> {
    hash: &'a str,
    size: i32,
    owner_ref: i32,
}

impl<'a, 'b> From<&'b Asset> for AssetRecord<'a>
where
    'b: 'a,
{
    fn from(a: &'b Asset) -> Self {
        AssetRecord {
            hash: &a.hash,
            size: a.size,
            owner_ref: a.owner_ref,
        }
    }
}

impl Asset {
    fn new(hash: String, size: i32) -> Self {
        Asset {
            id: 0,
            hash,
            size,
            owner_ref: 0,
            create_at: SystemTime::now(),
        }
    }
}
