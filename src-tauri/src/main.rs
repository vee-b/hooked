// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod database_helper; // Assuming `database_helper.rs` is in the same directory

use database_helper::{DatabaseHelper, Project};
use tauri::{Manager, State};
use tauri::command;
use mongodb::{Client, options::ClientOptions};
use mongodb::bson::doc;
use std::sync::Arc;
use tokio::sync::Mutex; // Mutex is needed for async code

#[tokio::main] // This enables the async entry point for the app.
async fn main() {
    // Set up MongoDB client asynchronously
    let mongo_uri = "***REMOVED***vness:8dSKcqijM7aVUH2V@hooked.1zbi6.mongodb.net/?retryWrites=true&w=majority&appName=Hooked";
    let mut client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    let server_api = mongodb::options::ServerApi::builder().version(mongodb::options::ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Initialize MongoDB client
    let client = Client::with_options(client_options).unwrap();
    //client_options.connect_timeout = Some(std::time::Duration::from_secs(10)); // Adjust timeout

    // Ping MongoDB to verify connection
    client.database("admin").run_command(doc! {"ping": 1}, None).await.unwrap();
    println!("Successfully connected to MongoDB!");

    // Initialize database helper asynchronously
    let db_helper = Arc::new(Mutex::new(DatabaseHelper::new().await.unwrap_or_else(|e| {
        eprintln!("Error initializing database: {}", e);
        std::process::exit(1); // Exit if database initialization fails
    })));

    // Initialize the Tauri app
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init()) // Uncomment if you need the Shell plugin
        .setup(move |app| {
            // Add MongoDB client and database helper to app's state
            app.manage(client);
            app.manage(db_helper.clone()); // Pass the Arc<Mutex<DatabaseHelper>> to the app

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            insert_project,
            get_all_projects,
            update_project,
            delete_project,
            get_sends_count,
            get_active_projects,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Example command function to insert a project
#[tauri::command]
async fn insert_project(
    db_helper: State<'_, DatabaseHelper>,
    project: Project,
) -> Result<(), String> {
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
    db_helper
        .get_all_projects()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_active_projects(db: State<'_, DatabaseHelper>) -> Result<Vec<Project>, String> {
    match db.get_active_projects().await {
        Ok(projects) => {
            println!("\nFetched active projects: {:?}", projects);
            Ok(projects)
        }
        Err(e) => {
            println!("\nError fetching active projects: {:?}", e);
            Err(e.to_string())
        }
    }
}

// Update a project
#[tauri::command]
async fn update_project(
    db_helper: State<'_, DatabaseHelper>,
    project: Project,
) -> Result<(), String> {
    db_helper
        .update_project(project)
        .await
        .map_err(|e| e.to_string())
}

// Delete a project
#[tauri::command]
async fn delete_project(db_helper: State<'_, DatabaseHelper>, id: String) -> Result<(), String> {
    db_helper
        .delete_project(&id)
        .await
        .map_err(|e| e.to_string())
}

// Get sends count
#[tauri::command]
async fn get_sends_count(db_helper: State<'_, DatabaseHelper>) -> Result<i64, String> {
    db_helper.get_sends_count().await.map_err(|e| e.to_string())
}






// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// #[cfg_attr(mobile, tauri::mobile_entry_point)]

// use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client, error::Result};

// #[tokio::main]
// async fn main() -> Result<()> {
//   let mut client_options =
//     ClientOptions::parse("***REMOVED***Vee:<644R8rnooVubZx9E>@hooked.akkpt.mongodb.net/?retryWrites=true&w=majority&appName=Hooked")
//       .await?;
//   // Set the server_api field of the client_options object to set the version of the Stable API on the client

//   let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
//   client_options.server_api = Some(server_api);
//   // Get a handle to the cluster
//   let client = Client::with_options(client_options)?;
//   // Ping the server to see if you can connect to the cluster
//   client
//     .database("admin")
//     .run_command(doc! {"ping": 1}, None)
//     .await?;
//   println!("Pinged your deployment. You successfully connected to MongoDB!");
//   Ok(())
// }


