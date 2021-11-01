// src/error_messages/model.rs

use crate::db;
use crate::error_handler::CustomError;
use crate::schema::val_error_messages;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize, AsChangeset, Insertable)]
#[table_name="val_error_messages"]
pub struct ValErrorMessage {
    pub username: String,
    pub body: String,
}
#[derive(Deserialize,Serialize, Queryable,Insertable)]
#[table_name="val_error_messages"]
pub struct ValErrorMessages {
    pub id: i32,
    pub username: String,
    pub body: String,
}

impl ValErrorMessages {
    pub fn find_all()-> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let val_error_messages = val_error_messages::table.load::<ValErrorMessages>(&conn)?;
        Ok(val_error_messages)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let val_error_message = val_error_messages::table.filter(val_error_messages::id.eq(id)).first(&conn)?;
        Ok(val_error_message)
    }

    pub fn create(val_error_message: ValErrorMessage) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let val_error_message = ValErrorMessage::from(val_error_message);
        let val_error_message = diesel::insert_into(val_error_messages::table)
            .values(val_error_message)
            .get_result(&conn)?;
        Ok(val_error_message)
    }
    pub fn update(id: i32, val_error_message: ValErrorMessage) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let val_error_message = diesel::update(val_error_messages::table)
            .filter(val_error_messages::id.eq(id))
            .set(val_error_message)
            .get_result(&conn)?;
        Ok(val_error_message)

    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(val_error_messages::table.filter(val_error_messages::id.eq(id))).execute(&conn)?;
        Ok(res)
    }

}

impl ValErrorMessage {
    fn from(val_error_message: ValErrorMessage) -> ValErrorMessage {
        ValErrorMessage {
            username: val_error_message.username,
            body: val_error_message.body,
        
        }
    }
}