// src/logs/model.rs

use crate::db;
use crate::error_handler::CustomError;
use crate::schema::logs;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize, AsChangeset, Insertable)]
#[table_name="logs"]
pub struct CreateLog {
    pub username: String,
    pub userid: String,
    pub environment: String,
    pub version: String,
    pub body: String,
}
#[derive(Deserialize,Serialize, Queryable,Insertable)]
#[table_name="logs"]
pub struct Logs {
    pub id: i32,
    pub username: String,
    pub userid: String,
    pub environment: String,
    pub version: String,
    pub body: String,
}

impl Logs {
    

    pub fn create(log: CreateLog) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let log = CreateLog::from(log);
        let log = diesel::insert_into(logs::table)
            .values(log)
            .get_result(&conn)?;
        Ok(log)
    }



}

