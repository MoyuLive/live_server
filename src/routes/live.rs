use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::json;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use tracing::{info, error, warn};

use crate::srs_callback::{SrsCallback, SrsResponse, SrsAuthRequest};
use crate::models::room::{Room, NewStreamLog};
use crate::models::schema::rooms::dsl::*;
use crate::models::schema::stream_logs;

pub type DbPool = diesel::r2d2::Pool<ConnectionManager<diesel::SqliteConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/srs/callback", post(srs_callback_handler))
        .route("/srs/auth", post(srs_auth_handler))
        .route("/health", get(health_check))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({"status": "ok"}))
}

async fn srs_callback_handler(
    State(state): State<AppState>,
    Json(callback): Json<SrsCallback>,
) -> Result<Json<SrsResponse>, StatusCode> {
    info!("Received SRS callback: {:?}", callback);
    
    let mut conn = state.pool.get().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    // 提取stream key
    let stream_key_value = callback.stream.clone();
    
    // 查找对应的直播间
    let room_result = rooms
        .filter(stream_key.eq(&stream_key_value))
        .first::<Room>(&mut conn)
        .optional();
    
    match room_result {
        Ok(Some(room)) => {
            // 记录流日志
            let log_entry = NewStreamLog::new(
                room.id,
                if callback.action.contains("publish") { "publish".to_string() } else { "play".to_string() },
                if callback.action.contains("connect") { "connect".to_string() } else { "disconnect".to_string() },
                callback.client_id,
                callback.ip,
                callback.stream_url,
            );
            
            match diesel::insert_into(stream_logs::table)
                .values(&log_entry)
                .execute(&mut conn)
            {
                Ok(_) => {
                    info!("Stream log recorded for room: {}", room.id);
                }
                Err(e) => {
                    error!("Failed to record stream log: {}", e);
                }
            }
            
            // 如果是推流连接，更新直播间状态
            if callback.action == "on_publish" {
                match diesel::update(rooms.find(room.id))
                    .set(status.eq(1)) // 1: 直播中
                    .execute(&mut conn)
                {
                    Ok(_) => {
                        info!("Room status updated to live: {}", room.id);
                    }
                    Err(e) => {
                        error!("Failed to update room status: {}", e);
                    }
                }
            } else if callback.action == "on_unpublish" {
                match diesel::update(rooms.find(room.id))
                    .set(status.eq(2)) // 2: 已结束
                    .execute(&mut conn)
                {
                    Ok(_) => {
                        info!("Room status updated to ended: {}", room.id);
                    }
                    Err(e) => {
                        error!("Failed to update room status: {}", e);
                    }
                }
            }
            
            Ok(Json(SrsResponse::success()))
        }
        Ok(None) => {
            warn!("Room not found for stream key: {:?}", stream_key_value);
            Ok(Json(SrsResponse::success())) // 即使房间不存在也返回成功，只记录日志
        }
        Err(e) => {
            error!("Database error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn srs_auth_handler(
    State(state): State<AppState>,
    Json(auth_request): Json<SrsAuthRequest>,
) -> Result<Json<SrsResponse>, StatusCode> {
    info!("Received SRS auth request: {:?}", auth_request);
    
    let mut conn = state.pool.get().map_err(|e| {
        error!("Failed to get database connection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    // 只对推流请求进行认证
    if auth_request.action == "publish" {
        if let Some(key) = auth_request.extract_stream_key() {
            match rooms
                .filter(stream_key.eq(&key))
                .first::<Room>(&mut conn)
                .optional()
            {
                Ok(Some(room)) => {
                    info!("Stream key validated for room: {}", room.id);
                    return Ok(Json(SrsResponse::success()));
                }
                Ok(None) => {
                    warn!("Invalid stream key: {:?}", key);
                    return Ok(Json(SrsResponse::error(1))); // 返回错误码，拒绝推流
                }
                Err(e) => {
                    error!("Database error during auth: {}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        } else {
            warn!("Could not extract stream key from URL: {}", auth_request.stream_url);
            return Ok(Json(SrsResponse::error(2))); // URL格式错误
        }
    }
    
    // 对于拉流请求，直接允许
    Ok(Json(SrsResponse::success()))
}
