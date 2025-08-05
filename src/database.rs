use diesel::connection::SimpleConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use anyhow::Result;

pub type DbPool = Pool<ConnectionManager<diesel::SqliteConnection>>;

pub fn establish_connection(database_url: &str) -> Result<DbPool> {
    let manager = ConnectionManager::<diesel::SqliteConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(15)
        .build(manager)?;
    
    Ok(pool)
}

pub fn run_migrations(pool: &DbPool) -> Result<()> {
    let mut conn = pool.get()?;
    
    // 创建表（如果不存在）
    conn.batch_execute("
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            hashed_password TEXT NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS rooms (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            stream_key TEXT NOT NULL,
            user_id INTEGER NOT NULL,
            status INTEGER NOT NULL DEFAULT 0,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE TABLE IF NOT EXISTS stream_logs (
            id INTEGER PRIMARY KEY,
            room_id INTEGER NOT NULL,
            stream_type TEXT NOT NULL,
            action TEXT NOT NULL,
            client_id TEXT NOT NULL,
            ip TEXT NOT NULL,
            url TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE INDEX IF NOT EXISTS idx_rooms_stream_key ON rooms(stream_key);
        CREATE INDEX IF NOT EXISTS idx_stream_logs_room_id ON stream_logs(room_id);
        CREATE INDEX IF NOT EXISTS idx_stream_logs_created_at ON stream_logs(created_at);
    ")?;
    
    Ok(())
}
