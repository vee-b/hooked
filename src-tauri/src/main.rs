// src-tauri/src/main.rs

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod database_helper; // Assuming `database_helper.rs` is in the same directory

use database_helper::{DatabaseHelper, Project};
use tauri::{Manager, State};
use mongodb::{Client as MongoClient, options::ClientOptions};
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::bson;
use futures_util::stream::TryStreamExt;
use std::sync::Arc;
use tokio::sync::Mutex; // Mutex is needed for async code
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct CloudinaryUploadResponse {
    secure_url: String,
}

#[tokio::main] // This enables the async entry point for the app.
async fn main() {
    // Set up MongoDB client asynchronously
    let mongo_uri = "***REMOVED***vness:8dSKcqijM7aVUH2V@hooked.1zbi6.mongodb.net/?retryWrites=true&w=majority&appName=Hooked";
    let mut client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    let server_api = mongodb::options::ServerApi::builder().version(mongodb::options::ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Initialize MongoDB client
    let client = MongoClient::with_options(client_options).unwrap();
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
            upload_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn insert_project(client: State<'_, MongoClient>, project: Project) -> Result<(), String> {
    let collection = client.database("hooked_db").collection("projects");
    match collection.insert_one(bson::to_document(&project).unwrap(), None).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn get_all_projects(client: State<'_, MongoClient>) -> Result<Vec<Project>, String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");
    let mut cursor = collection.find(None, None).await.map_err(|e| e.to_string())?;
    let mut projects = Vec::new();

    while let Some(doc) = cursor.try_next().await.map_err(|e| e.to_string())? {
        let project: Project = bson::from_document(doc).map_err(|e| e.to_string())?;
        projects.push(project);
    }
    Ok(projects)
}

#[tauri::command]
async fn get_active_projects(client: State<'_, MongoClient>) -> Result<Vec<Project>, String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");
    let filter = doc! {"is_active": 1};
    let mut cursor = collection.find(filter, None).await.map_err(|e| e.to_string())?;
    let mut projects = Vec::new();

    while let Some(doc) = cursor.try_next().await.map_err(|e| e.to_string())? {
        let project: Project = bson::from_document(doc).map_err(|e| e.to_string())?;
        projects.push(project);
    }
    Ok(projects)
}

#[tauri::command]
async fn update_project(client: State<'_, MongoClient>, project: Project) -> Result<(), String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");

    if let Some(_id) = project._id {
        let filter = doc! {"_id": _id};
        let update = doc! {"$set": bson::to_document(&project).unwrap()};
        collection.update_one(filter, update, None).await.map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Project ID is required for update".to_string())
    }
}

#[tauri::command]
async fn delete_project(client: State<'_, MongoClient>, _id: String) -> Result<(), String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");
    let object_id = bson::oid::ObjectId::parse_str(&_id).map_err(|e| e.to_string())?;
    let filter = doc! {"_id": object_id};

    collection.delete_one(filter, None).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_sends_count(client: State<'_, MongoClient>) -> Result<i64, String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");
    let count = collection.count_documents(None, None).await.map_err(|e| e.to_string())?;
    Ok(count.try_into().map_err(|_| "Count exceeds i64 capacity".to_string())?)
}

#[tauri::command]
async fn upload_image(image_data: Vec<u8>, image_name: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    let cloud_name = "du9hsgxds";
    let upload_preset = "shafaedyn";

    let part = reqwest::multipart::Part::bytes(image_data)
        .file_name(image_name);  // Provide the image name (filename)

    let form = reqwest::multipart::Form::new()
        .part("file", part)  // Add the image part
        .text("upload_preset", upload_preset.to_string());  // Add the upload preset

    let res = client
        .post(format!("https://api.cloudinary.com/v1_1/{}/upload", cloud_name))  // Replace with your Cloudinary cloud name
        .multipart(form)  // Use multipart to send the file
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let body = res.text().await.map_err(|e| format!("Failed to read response: {}", e))?;

    // Check the response for a successful upload (this will depend on Cloudinary's response format)
    if !body.contains("secure_url") {
        return Err("Image upload failed".to_string());
    }

    // Assuming Cloudinary returns a URL in the response
    Ok(body)  // Return the Cloudinary URL in the response
}
