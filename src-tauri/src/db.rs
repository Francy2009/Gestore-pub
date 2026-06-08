use sqlx::{sqlite::SqlitePoolOptions, SqlitePool, Row, migrate::MigrateDatabase};
use std::path::PathBuf;
use tauri::AppHandle;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use crate::error::{Result, AppError};

pub struct Database {
    pool: SqlitePool,
    app_handle: AppHandle,
}

impl Database {
    pub fn new(app_handle: &AppHandle) -> Result<Self> {
        let db_path = Self::get_db_path(app_handle)?;
        
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let database_url = format!("sqlite://{}?mode=rwc", db_path.display());
        
        // Create database if it doesn't exist
        if !sqlx::Sqlite::database_exists(&database_url).await.unwrap_or(false) {
            sqlx::Sqlite::create_database(&database_url).await?;
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url).await?;

        Ok(Self {
            pool,
            app_handle: app_handle.clone(),
        })
    }

    pub fn get_db_path(app_handle: &AppHandle) -> Result<PathBuf> {
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| AppError::Io(std::io::Error::other(e)))?;
        
        Ok(app_data_dir.join("gestore-pub.db"))
    }

    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    pub async fn ensure_admin_exists(&self) -> Result<()> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM members WHERE id IN (SELECT member_id FROM user_roles WHERE role = 'admin')"
        )
        .fetch_one(&self.pool)
        .await?;

        if count == 0 {
            // Create default admin
            let admin_id = Uuid::new_v4().to_string();
            let role_id = Uuid::new_v4().to_string();
            let now = Utc::now();
            
            let mut tx = self.pool.begin().await?;
            
            sqlx::query(
                r#"
                INSERT INTO members (id, first_name, last_name, username, password, joined_at, password_changed, must_setup, role_id)
                VALUES (?, 'Admin', 'Admin', 'admin', ?, ?, 0, 1, ?)
                "#
            )
            .bind(&admin_id)
            .bind(&Self::hash_password("admin123!")?)
            .bind(now)
            .bind(&role_id)
            .execute(&mut *tx)
            .await?;

            sqlx::query(
                "INSERT INTO user_roles (id, role, member_id) VALUES (?, 'admin', ?)"
            )
            .bind(&role_id)
            .bind(&admin_id)
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;
            tracing::info!("Default admin created: admin / admin123!");
        }

        Ok(())
    }

    fn hash_password(password: &str) -> Result<String> {
        let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
        Ok(hash)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        Ok(bcrypt::verify(password, hash)?)
    }

    // Getters for commands
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

// Helper types for queries
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Member {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub member_number: Option<String>,
    pub qr_token: Option<String>,
    pub username: String,
    pub password: String,
    pub joined_at: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub password_changed: bool,
    pub must_setup: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MemberWithRole {
    #[serde(flatten)]
    pub member: Member,
    pub role: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Attendance {
    pub id: String,
    pub member_id: Option<String>,
    pub check_in_time: DateTime<Utc>,
    pub check_in_day: String,
    pub member_first_name: String,
    pub member_last_name: String,
    pub member_number: String,
    pub member_was_deleted: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Session {
    pub id: String,
    pub token_hash: String,
    pub member_id: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}