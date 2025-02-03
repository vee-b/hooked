// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]

mod database_helper;  // Assuming `database_helper.rs` is in the same directory
use database_helper::{Project, DatabaseHelper};
use tauri::{Manager, State};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Convert this setup closure to an async function
            let db_helper = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(DatabaseHelper::new()) // Wait for the async call to complete
                .unwrap_or_else(|e| {
                    eprintln!("Error initializing database: {}", e);
                    std::process::exit(1); // Exit if database initialization fails
                });

            app.manage(db_helper);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            insert_project, // Add your command functions here
            get_all_projects,
            update_project,
            delete_project,
            get_sends_count,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Example command function to insert a project
#[tauri::command]
async fn insert_project(db_helper: State<'_, DatabaseHelper>, project: Project) -> Result<(), String> {
    println!("Inserting project: {:?}", project); // Log project details
    db_helper.insert_project(project).await.map_err(|e| {
        eprintln!("Failed to insert project: {}", e); // Log the error
        e.to_string()
    })?;
    Ok(())
}

// Get all projects from the database
#[tauri::command]
async fn get_all_projects(db_helper: State<'_, DatabaseHelper>) -> Result<Vec<Project>, String> {
    db_helper.get_all_projects().await.map_err(|e| e.to_string())
}

// Update a project
#[tauri::command]
async fn update_project(db_helper: State<'_, DatabaseHelper>, project: Project) -> Result<(), String> {
    db_helper.update_project(project).await.map_err(|e| e.to_string())
}

// Delete a project
#[tauri::command]
async fn delete_project(db_helper: State<'_, DatabaseHelper>, id: String) -> Result<(), String> {
    db_helper.delete_project(&id).await.map_err(|e| e.to_string())
}

// Get the count of projects
#[tauri::command]
async fn get_sends_count(db_helper: State<'_, DatabaseHelper>) -> Result<i64, String> {
    db_helper.get_sends_count().await.map_err(|e| e.to_string())
}