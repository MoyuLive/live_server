use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Room {
    pub id: i64,
    pub name: String,
    pub stream_key: String,
    pub user_id: i64,
    pub status: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::models::schema::rooms)]
pub struct NewRoom {
    pub name: String,
    pub stream_key: String,
    pub user_id: i64,
    pub status: i32,
}

impl NewRoom {
    pub fn new(name: String, stream_key: String, user_id: i64) -> Self {
        Self {
            name,
            stream_key,
            user_id,
            status: 0, // 0: 未开始, 1: 直播中, 2: 已结束
        }
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct StreamLog {
    pub id: i64,
    pub room_id: i64,
    pub stream_type: String, // "publish" 或 "play"
    pub action: String,      // "connect" 或 "disconnect"
    pub client_id: String,
    pub ip: String,
    pub url: String,
    pub created_at: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::models::schema::stream_logs)]
pub struct NewStreamLog {
    pub room_id: i64,
    pub stream_type: String,
    pub action: String,
    pub client_id: String,
    pub ip: String,
    pub url: String,
}

impl NewStreamLog {
    pub fn new(
        room_id: i64,
        stream_type: String,
        action: String,
        client_id: String,
        ip: String,
        url: String,
    ) -> Self {
        Self {
            room_id,
            stream_type,
            action,
            client_id,
            ip,
            url,
        }
    }
}
