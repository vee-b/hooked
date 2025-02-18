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
use serde_json::Value;
//use chrono::Utc;
use std::collections::HashMap;
use sha1::{Sha1, Digest};

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

    // Get the image_path before deleting from MongoDB
    let project_result = collection.find_one(filter.clone(), None).await.map_err(|e| e.to_string())?;
    let image_path = match project_result {
        Some(doc) => {
            match doc.get_str("image_path") {
                Ok(path) => Some(path.to_string()),
                Err(_) => None,
            }
        },
        None => None,
    };

    // Delete from MongoDB
    collection.delete_one(filter, None).await.map_err(|e| e.to_string())?;

    // Delete from Cloudinary (if image_path exists)
    if let Some(path) = image_path {
        if let Err(e) = delete_from_cloudinary(path).await {
            // Log the error but don't return it to avoid blocking the MongoDB delete
            eprintln!("Error deleting from Cloudinary: {}", e); 
        }
    }

    Ok(())
}

async fn delete_from_cloudinary(image_url: String) -> Result<(), String> {
    // Extract the public_id from the image URL
    let public_id = extract_cloudinary_public_id(&image_url)?; // Implement this function (see below)


    let cloud_name = "du9hsgxds"; // Replace
    let api_key = "896781979879415";       // Replace
    let api_secret = "X-xII6Q4WQkXfjV68MCLFKPq5hQ"; // Replace

    let timestamp = chrono::Utc::now().timestamp().to_string(); // For signing the request
    let signature_string = format!("public_id={}&timestamp={}{}", public_id, timestamp, api_secret);
    
    // Create a Sha1 hasher
    let mut hasher = Sha1::new();
    hasher.update(signature_string);
    let result = hasher.finalize();
    let signature = format!("{:x}", result); // Convert to hex string

    let client = reqwest::Client::new();
    let mut params = HashMap::new();
    params.insert("public_id", public_id);
    params.insert("api_key", api_key.to_string());
    params.insert("timestamp", timestamp);
    params.insert("signature", signature);

    let res = client.post(format!("https://api.cloudinary.com/v1_1/{}/image/destroy", cloud_name))
        .form(&params)
        .send()
        .await
        .map_err(|err| format!("Error sending request: {}", err))?;

    if res.status().is_success() {
        println!("Successfully deleted image from Cloudinary: {}", image_url);
        Ok(())
    } else {
        let error_body = res.text().await.unwrap_or("Failed to get error body".to_string());
        let val: Value = serde_json::from_str(&error_body).map_err(|e| e.to_string())?;
        let err_msg = val["error"]["message"].as_str().unwrap_or("Unknown Cloudinary Error").to_string();

        Err(format!("Failed to delete image from Cloudinary: {}. Cloudinary response: {}", image_url, err_msg))
    }
}


fn extract_cloudinary_public_id(image_url: &str) -> Result<String, String> {
    // (Implementation depends on your Cloudinary URL structure. Example below.)
    // Example assuming URL format: https://res.cloudinary.com/cloud_name/image/upload/v1678886400/public_id.jpg
    let parts: Vec<&str> = image_url.split('/').collect();
    if parts.len() >= 8 { // Adjust as needed
        let public_id_with_ext = parts[7];
        let public_id = public_id_with_ext.split('.').next().unwrap_or(public_id_with_ext); // Extract up to .ext

        Ok(public_id.to_owned())
    } else {
        Err("Invalid Cloudinary URL format".to_owned())
    }
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
    let cloud_name = "du9hsgxds"; // Replace with your Cloudinary cloud name
    let upload_preset = "shafaedyn"; // Replace with your upload preset

    let part = reqwest::multipart::Part::bytes(image_data)
        .file_name(image_name);

    let form = reqwest::multipart::Form::new()
        .part("file", part)
        .text("upload_preset", upload_preset.to_string());

    let res = client
        .post(format!("https://api.cloudinary.com/v1_1/{}/upload", cloud_name))
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if res.status().is_success() {
        let response_body: Value = res.json().await.map_err(|e| format!("Failed to parse JSON: {}", e))?;
        if let Some(secure_url) = response_body.get("secure_url").and_then(Value::as_str) {
            Ok(secure_url.to_string())
        } else {
            Err("`secure_url` not found in Cloudinary response".to_string())
        }
    } else {
        let status = res.status();
        let error_body = res.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
        Err(format!("Cloudinary upload failed (Status: {}): {}", status, error_body))
    }
}
