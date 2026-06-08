use tauri::Manager;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod commands;
mod db;
mod auth;
mod error;

use commands::*;
use db::Database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,tauri=warn".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // Initialize database
            let app_handle = app.handle().clone();
            let db = Database::new(&app_handle)?;
            app.manage(db);

            // Run migrations
            let db_state = app.state::<Database>();
            tauri::async_runtime::block_on(async move {
                db_state.run_migrations().await.expect("Failed to run migrations");
            });

            // Initialize default admin if needed
            let db_state = app.state::<Database>();
            tauri::async_runtime::block_on(async move {
                db_state.ensure_admin_exists().await.expect("Failed to ensure admin exists");
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Auth
            auth_login,
            auth_logout,
            auth_get_current_user,
            auth_setup_admin,
            auth_change_password,
            // Members
            members_get_all,
            members_get_checkin_list,
            members_create,
            members_renew,
            members_delete,
            // Attendance
            attendance_register,
            attendance_get_today,
            attendance_get_logs,
            attendance_delete,
            // Summary
            summary_get_monthly,
            // Backup
            backup_export,
            backup_restore,
            // Settings
            settings_get_db_path,
            settings_export_db,
            settings_import_db,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}