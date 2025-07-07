// src-tauri/src/main.rs

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Ensures compatibility with mobile devices if the mobile feature is enabled.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
// Import database_helper (assuming `database_helper.rs` is in the same directory)
mod database_helper; 

use database_helper::{DatabaseHelper, Project, Account};
use tauri::{Manager, State}; // Manager: Provides app management features like accessing state. State: Allows sharing state (like database connections) between Tauri commands.
use mongodb::{Client as MongoClient, options::ClientOptions}; // MongoClient: The main MongoDB client for database interactions. ClientOptions: For configuring MongoDB connection options.
use mongodb::bson::{self, Document, oid::ObjectId}; // bson: MongoDB’s binary JSON format. doc: Macro for creating BSON documents. 
use bson::{Bson, doc};
use futures_util::stream::TryStreamExt; // Provides asynchronous streaming methods.
use std::sync::Arc; // Enables thread-safe reference counting.
use tokio::sync::Mutex; // Allows safe sharing and mutation of data in async code.
use serde::{Serialize, Deserialize}; // Used for converting Rust structs to/from JSON.
use serde_json::Value; // Represents arbitrary JSON data.
use std::collections::HashMap;
use sha1::{Sha1, Digest}; // Imports the SHA-1 hashing algorithm for generating Cloudinary signatures.

#[derive(Serialize, Deserialize)]
// Struct to deserialize Cloudinary's upload response.
struct CloudinaryUploadResponse {
    secure_url: String, // Image URL.
}

// Mark the main function as asynchronous using the tokio runtime.
#[tokio::main] // Enables the async entry point for the app.
async fn main() {
    // Set up MongoDB client asynchronously.
    // MongoDB connection string.
    let mongo_uri = "***REMOVED***vness:8dSKcqijM7aVUH2V@hooked.1zbi6.mongodb.net/?retryWrites=true&w=majority&appName=Hooked";
    // Parse MongoDB connection options asynchronously.
    let mut client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    // Sets the MongoDB API version to V1 for stability.
    let server_api = mongodb::options::ServerApi::builder().version(mongodb::options::ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Initialize MongoDB client
    let client = MongoClient::with_options(client_options).unwrap();
    //client_options.connect_timeout = Some(std::time::Duration::from_secs(10)); // Adjust timeout

    // Ping MongoDB to verify connection
    client.database("admin").run_command(doc! {"ping": 1}, None).await.unwrap();
    println!("Successfully connected to MongoDB!");

    // Initialize database helper asynchronously (wraps it in a thread-safe Arc<Mutex>, and exits on failure.)
    let db_helper = Arc::new(Mutex::new(DatabaseHelper::new().await.unwrap_or_else(|e| {
        eprintln!("Error initializing database: {}", e);
        std::process::exit(1); // Exit if database initialization fails
    })));

    // Initialize the Tauri app builder
    tauri::Builder::default()
        // Add the logging and shell plugins for debugging and executing shell commands.
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        // Add the MongoDB client and DatabaseHelper to Tauri’s state.
        .setup(move |app| {
            // Add MongoDB client and database helper to app's state
            app.manage(client);
            app.manage(db_helper.clone()); // Pass the Arc<Mutex<DatabaseHelper>> to the app

            Ok(())
        })
        // Register the Tauri commands that the frontend can invoke.
        .invoke_handler(tauri::generate_handler![
            insert_project,
            get_all_projects,
            update_project,
            save_annotations,
            delete_project,
            get_sends_count,
            get_sends_summary,
            get_styles_summary,
            get_active_projects,
            get_inactive_projects,
            get_active_filtered_projects,
            get_inactive_filtered_projects,
            upload_image,
            get_project_by_id,
            create_account,
            login,
        ])
        // Start the Tauri application.
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Inserts a new Project document into the projects collection.
#[tauri::command] // Marks the function as a Tauri command, allowing the frontend (e.g., SvelteKit) to invoke the function asynchronously.
// client: State<'_, MongoClient>: State: This is Tauri's way of sharing state across different commands.MongoClient: The MongoDB client instance, which provides access to the database. '_': A lifetime specifier. This indicates that the MongoClient reference is tied to the application's state lifetime. 
async fn insert_project(client: State<'_, MongoClient>, project: Project) -> Result<(), String> {
    let collection = client.database("hooked_db").collection("projects");

    // Convert project to BSON document
    let doc = match bson::to_document(&project) {
        Ok(doc) => doc,
        Err(e) => return Err(format!("Serialization error: {}", e)),
    };

    // Insert into MongoDB
    match collection.insert_one(doc, None).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

// Fetches all project documents from the database and converts them to Project objects.
#[tauri::command]
// Returns a vector (list) of Project objects.
async fn get_all_projects(client: State<'_, MongoClient>) -> Result<Vec<Project>, String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");
    // Performs a find query with no filter (None means "match everything"). The second None is for additional options (e.g., sorting or projection).
    let mut cursor = collection.find(None, None).await.map_err(|e| e.to_string())?; // The ? operator propagates the error, exiting the function early if an error occurs.
    // Initialize an empty Vec (vector) to hold the list of Project objects.
    let mut projects = Vec::new();

    // while let Some(doc): Loops through the cursor while there are documents available. try_next(): from the futures crate and returns a Result<Option<Document>>.
    while let Some(doc) = cursor.try_next().await.map_err(|e| e.to_string())? {
        let project: Project = bson::from_document(doc).map_err(|e| e.to_string())?;
        // Add the converted Project struct to the projects vector.
        projects.push(project);
    }
    // Send the list of all projects to the frontend.
    Ok(projects)
}

// Fetches all project where is_active is 1.
#[tauri::command]
async fn get_active_projects(client: State<'_, MongoClient>) -> Result<Vec<Project>, String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");
    // Retrieve only active projects (where is_active is set to 1). doc!: This macro from the mongodb crate helps create BSON documents in a concise way. {"is_active": 1}: This is the MongoDB filter.
    let filter = doc! {"is_active": 1};
    // Queries the database for all documents matching the filter.
    let mut cursor = collection.find(filter, None).await.map_err(|e| e.to_string())?;
    let mut projects = Vec::new();

    while let Some(doc) = cursor.try_next().await.map_err(|e| e.to_string())? {
        let project: Project = bson::from_document(doc).map_err(|e| e.to_string())?;
        projects.push(project);
    }
    Ok(projects)
}

// Fetches all project where is_active is 0.
#[tauri::command]
async fn get_inactive_projects(client: State<'_, MongoClient>) -> Result<Vec<Project>, String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");
    // Retrieve only active projects (where is_active is set to 0).
    let filter = doc! {"is_active": 0};
    let mut cursor = collection.find(filter, None).await.map_err(|e| e.to_string())?;
    let mut projects = Vec::new();

    while let Some(doc) = cursor.try_next().await.map_err(|e| e.to_string())? {
        let project: Project = bson::from_document(doc).map_err(|e| e.to_string())?;
        projects.push(project);
    }
    Ok(projects)
}

// Fetches all projects where is_active is 1 with optional filtering
#[tauri::command]
async fn get_active_filtered_projects(
    client: State<'_, MongoClient>,
    // grade: Option<String>,
    grades: Option<Vec<String>>,
    sent_status: Option<String>,
    styles: Option<Vec<String>>,
) -> Result<Vec<Project>, String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");

    // Base filter: Only return active projects
    let mut filter = doc! { "is_active": 1 };

    // Add grade filter if provided
    if let Some(grades_list) = grades {
        if !grades_list.is_empty() {
            filter.insert("grade", doc! { "$in": grades_list }); // Use $in to match multiple grades
        }
    }

    if let Some(styles_list) = styles {
        if !styles_list.is_empty() {
            filter.insert("style", doc! { "$in": styles_list });
        }
    }

    // Add sent_status filter if provided
    if let Some(s) = sent_status {
        if s == "true" || s == "false" {
            let sent_value = if s == "true" { 1 } else { 0 };
            filter.insert("is_sent", Bson::Int32(sent_value));
        }
        // If s is empty or anything else, skip filtering on is_sent
    }

    // Log the final MongoDB query for debugging
    println!("MongoDB Query: {:?}", filter);

    // Execute the query
    let mut cursor = collection.find(filter, None).await.map_err(|e| e.to_string())?;
    let mut projects = Vec::new();

    while let Some(doc) = cursor.try_next().await.map_err(|e| e.to_string())? {
        let project: Project = bson::from_document(doc).map_err(|e| e.to_string())?;
        projects.push(project);
    }

    println!("Projects returned: {:?}", projects.len()); // Debugging
    Ok(projects)
}

// Fetches all projects where is_active is 0 with optional filtering
#[tauri::command]
async fn get_inactive_filtered_projects(
    client: State<'_, MongoClient>,
    // grade: Option<String>,
    grades: Option<Vec<String>>,
    sent_status: Option<String>,
    styles: Option<Vec<String>>,
) -> Result<Vec<Project>, String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");

    // Base filter: Only return active projects
    let mut filter = doc! { "is_active": 0 };

    // Add grade filter if provided
    if let Some(grades_list) = grades {
        if !grades_list.is_empty() {
            filter.insert("grade", doc! { "$in": grades_list }); // Use $in to match multiple grades
        }
    }

    if let Some(styles_list) = styles {
        if !styles_list.is_empty() {
            filter.insert("style", doc! { "$in": styles_list });
        }
    }
    
    // Add sent_status filter if provided
    if let Some(s) = sent_status {
        if s == "true" || s == "false" {
            let sent_value = if s == "true" { 1 } else { 0 };
            filter.insert("is_sent", Bson::Int32(sent_value));
        }
        // If s is empty or anything else, skip filtering on is_sent
    }

    // Log the final MongoDB query for debugging
    println!("MongoDB Query: {:?}", filter);

    // Execute the query
    let mut cursor = collection.find(filter, None).await.map_err(|e| e.to_string())?;
    let mut projects = Vec::new();

    while let Some(doc) = cursor.try_next().await.map_err(|e| e.to_string())? {
        let project: Project = bson::from_document(doc).map_err(|e| e.to_string())?;
        projects.push(project);
    }

    println!("Projects returned: {:?}", projects.len()); // Debugging
    Ok(projects)
}

// Updates a project by _id if it exists.
#[tauri::command]
async fn update_project(client: State<'_, MongoClient>, project: Project) -> Result<(), String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");

    // Log received project data
    println!("Received project data: {:?}", project);

    if let Some(_id) = project._id {
        let filter = doc! {"_id": _id};

        // Convert Rust struct into a BSON document
        let mut update_doc = bson::to_document(&project).map_err(|e| e.to_string())?;

        // Ensure `date_time` is a proper integer
        if let Some(bson::Bson::Int64(date_time)) = update_doc.get("date_time") {
            update_doc.insert("date_time", bson::Bson::Int64(*date_time));
        } else {
            return Err("Invalid date_time format".to_string());
        }

        // Ensure boolean fields (`is_active` and `is_sent`) are stored as `i32`
        if let Some(bson::Bson::Boolean(is_active)) = update_doc.get("is_active") {
            update_doc.insert("is_active", bson::Bson::Int32(if *is_active { 1 } else { 0 }));
        }
        if let Some(bson::Bson::Boolean(is_sent)) = update_doc.get("is_sent") {
            update_doc.insert("is_sent", bson::Bson::Int32(if *is_sent { 1 } else { 0 }));
        }

        // // Ensure `coordinates` field is properly handled as an array of floats
        // if let Some(bson::Bson::Array(coordinates)) = update_doc.get("coordinates") {
        //     // Log the received coordinates
        //     println!("Received coordinates: {:?}", coordinates);
            
        //     // We assume that coordinates is an array of numbers (floats or integers).
        //     let coordinates: Vec<f64> = coordinates.iter()
        //         .filter_map(|bson| match bson {
        //             bson::Bson::Double(val) => Some(*val),
        //             bson::Bson::Int32(val) => Some(*val as f64),
        //             bson::Bson::Int64(val) => Some(*val as f64),
        //             _ => None, // Ignore invalid types
        //         })
        //         .collect();

        //     // Log the coordinates before updating
        //     println!("Processed coordinates: {:?}", coordinates);
            
        //     update_doc.insert("coordinates", bson::Bson::Array(coordinates.iter().map(|&x| bson::Bson::Double(x)).collect()));
        // }

        // Handle `coordinates` only if valid array of objects
        let valid_coordinates = match update_doc.get("coordinates") {
            Some(bson::Bson::Array(array)) if !array.is_empty() => {
                // Log and preserve array as-is
                println!("Valid coordinates received: {:?}", array);
                true
            }
            _ => false,
        };

        // If not valid, fetch existing coordinates and use them
        if !valid_coordinates {
            if let Ok(existing_doc) = collection.find_one(filter.clone(), None).await {
                if let Some(existing_coordinates) = existing_doc
                    .and_then(|doc| doc.get("coordinates").cloned())
                {
                    println!("Reusing existing coordinates: {:?}", existing_coordinates);
                    update_doc.insert("coordinates", existing_coordinates);
                }
            }
        }

        let update = doc! {"$set": update_doc};

        match collection.update_one(filter, update, None).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Project ID is required for update".to_string())
    }
}

// Define the SaveAnnotationsRequest struct
#[derive(Serialize, Deserialize, Debug)]
pub struct SaveAnnotationsRequest {
    pub project_id: String,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotation {
    pub lat: f64,
    pub lng: f64,
    pub note: Vec<String>,
}

// Updates the annotations for a project by _id if it exists.
#[tauri::command]
async fn save_annotations(client: State<'_, MongoClient>, request: SaveAnnotationsRequest) -> Result<(), String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");

    // Log received project annotations data
    println!("Received project annotations: {:?}", request);

    let project_id = &request.project_id;
    if !project_id.is_empty() {
        let filter = doc! { "_id": bson::oid::ObjectId::parse_str(project_id).map_err(|e| e.to_string())? };

        // Convert annotations to BSON based on their number type
        let annotations: Vec<Bson> = request.annotations.iter().map(|annotation| {
            Bson::Document(doc! {
                "lat": annotation.lat,
                "lng": annotation.lng,
                "note": &annotation.note
            })
        }).collect();

        let update_doc = doc! {
            "$set": {
                "coordinates": annotations,
            },
        };

        match collection.update_one(filter, update_doc, None).await {
            Ok(_) => {
                println!("Annotations for project {} saved successfully!", project_id);
                Ok(())
            },
            Err(e) => {
                println!("Failed to save annotations for project {}: {}", project_id, e);
                Err(e.to_string())
            }
        }
    } else {
        Err("Project ID is required".to_string())
    }
}

// Deletes a project and associated Cloudinary image.
#[tauri::command]
async fn delete_project(client: State<'_, MongoClient>, _id: String) -> Result<(), String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");
    // Parse the _id to ObjectId with 'bson::oid::ObjectId::parse_str(&_id)'.
    let object_id = bson::oid::ObjectId::parse_str(&_id).map_err(|e| e.to_string())?;
    let filter = doc! {"_id": object_id};

    // Get the image_path before deleting from MongoDB
    let project_result = collection.find_one(filter.clone(), None).await.map_err(|e| e.to_string())?; // .clone(): Creates a copy of the filter so we can reuse it later.
    // Extract the image path
    let image_path = match project_result { // match project_result: If a document is found (Some(doc)), attempt to extract the "image_path" field.
        Some(doc) => {
            match doc.get_str("image_path") {  // Retrieves the "image_path" as a string.
                Ok(path) => Some(path.to_string()), // If successful, wrap it in Some(path.to_string()).
                Err(_) => None, // If the field does not exist or cannot be read, return None.
            }
        },
        None => None,
    };

    // Delete from MongoDB
    collection.delete_one(filter, None).await.map_err(|e| e.to_string())?;

    // Delete from Cloudinary (if image_path exists)
    if let Some(path) = image_path { // if let Some(path): Checks if an image_path was found.
        if let Err(e) = delete_from_cloudinary(path).await {
            // Log the error but don't return it to avoid blocking the MongoDB delete
            eprintln!("Error deleting from Cloudinary: {}", e); 
        }
    }

    Ok(())
}

// Extracts the public_id and deletes the image using a signed request.
async fn delete_from_cloudinary(image_url: String) -> Result<(), String> {
    // Extract the public_id (necessary to identify and delete the image from Cloudinary) from the image URL.
    let public_id = extract_cloudinary_public_id(&image_url)?;

    let cloud_name = "du9hsgxds";
    let api_key = "896781979879415";
    let api_secret = "X-xII6Q4WQkXfjV68MCLFKPq5hQ";

    // Generate a secure signature using the current time.
    let timestamp = chrono::Utc::now().timestamp().to_string(); // For signing the request
    // Construct the signature base string required by Cloudinary’s API for request validation.
    let signature_string = format!("public_id={}&timestamp={}{}", public_id, timestamp, api_secret);
    
    // Create a Sha1 hasher
    let mut hasher = Sha1::new();
    hasher.update(signature_string);
    let result = hasher.finalize();
    let signature = format!("{:x}", result); // Convert to hex string

    // Prepare the request data.
    let client = reqwest::Client::new(); // Creates a new HTTP client to send the POST request.
    let mut params = HashMap::new(); // HashMap::new(): Creates a new map to store the form data.
    // Add the required parameters.
    params.insert("public_id", public_id);
    params.insert("api_key", api_key.to_string());
    params.insert("timestamp", timestamp);
    params.insert("signature", signature);

    // Send the request to delete the image from Cloudinary.
    let res = client.post(format!("https://api.cloudinary.com/v1_1/{}/image/destroy", cloud_name))
        .form(&params) 
        .send()
        .await
        .map_err(|err| format!("Error sending request: {}", err))?;

    // Handle the response
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

// Parses the public ID from the Cloudinary URL.
fn extract_cloudinary_public_id(image_url: &str) -> Result<String, String> {
    // Split the URL into parts. image_url.split('/'): Splits the input URL by the / character, producing an iterator of substrings. .collect(): Collects the resulting substrings into a Vec<&str> (a vector of string slices). parts: A vector containing each section of the URL split by /.
    let parts: Vec<&str> = image_url.split('/').collect();
    // Validate the URL structure.
    if parts.len() >= 8 { // Adjust as needed (if the URL structure changes, this index may need to be updated).
        let public_id_with_ext = parts[7]; // Extract the public ID. public_id_with_ext: Holds the image's public ID with the file extension.
        let public_id = public_id_with_ext.split('.').next().unwrap_or(public_id_with_ext); // Remove the file extension (extract up to .ext).

        // Return the extracted public ID.
        Ok(public_id.to_owned()) // .to_owned(): Converts the &str (borrowed reference) into a String (owned data).
    } else {
        Err("Invalid Cloudinary URL format".to_owned())
    }
}

// Returns the total number of documents in the projects collection.
#[tauri::command]
async fn get_sends_count(client: State<'_, MongoClient>) -> Result<i64, String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");
    // Count all documents in the collection. 
    let count = collection.count_documents(None, None).await.map_err(|e| e.to_string())?;
    // Convert the count into an i64 (signed 64-bit integer).
    Ok(count.try_into().map_err(|_| "Count exceeds i64 capacity".to_string())?)
}

// Returns the total sends and sends count by grade.
#[tauri::command]
async fn get_sends_summary(client: State<'_, MongoClient>) -> Result<(i64, Vec<(String, i64)>), String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");

    // Log the number of matching projects before aggregation
    let matching_count = collection.count_documents(doc! { "is_sent": 1 }, None).await.map_err(|e| e.to_string())?;
    println!("Matching projects with is_sent = 1: {}", matching_count);

    // Aggregation pipeline: filter sent projects and group by grade.
    let pipeline = vec![
        doc! { "$match": { "is_sent": 1 } },
        doc! { "$group": { "_id": "$grade", "count": { "$sum": 1 } } }
    ];

    // Execute the aggregation pipeline.
    let mut cursor = collection.aggregate(pipeline, None).await.map_err(|e| e.to_string())?;

    let mut grade_counts = Vec::new();
    let mut total_count = 0;

    // Iterate through the results.
    while let Some(doc) = cursor.try_next().await.map_err(|e| e.to_string())? {
        println!("Aggregation result: {:?}", doc);
        
        if let Some(grade) = doc.get_str("_id").ok() {
            // Handle both Int32 and Int64
            let count = match doc.get("count") {
                Some(bson::Bson::Int32(n)) => *n as i64,
                Some(bson::Bson::Int64(n)) => *n,
                _ => 0,
            };
            
            grade_counts.push((grade.to_string(), count));
            total_count += count;
        }
    }

    // Log the result before returning
    println!("Returning sends summary: total_count = {}, grade_counts = {:?}", total_count, grade_counts);

    Ok((total_count, grade_counts))
}

#[tauri::command]
async fn get_styles_summary(client: State<'_, MongoClient>) -> Result<Vec<(String, i64, i64)>, String> {
    let collection = client.database("hooked_db").collection::<Document>("projects");

    // Pipeline to group by style for done (is_sent = 1)
    let done_pipeline = vec![
        doc! { "$match": { "is_sent": 1 }},
        doc! { "$unwind": "$style" },  // break style array into multiple docs
        doc! { "$group": { "_id": "$style", "count": { "$sum": 1 }}}
    ];

    let mut done_cursor = collection.aggregate(done_pipeline, None).await.map_err(|e| e.to_string())?;
    let mut done_counts = std::collections::HashMap::new();
    while let Some(doc) = done_cursor.try_next().await.map_err(|e| e.to_string())? {
        if let Some(style) = doc.get_str("_id").ok() {
            let count = match doc.get("count") {
                Some(bson::Bson::Int32(n)) => *n as i64,
                Some(bson::Bson::Int64(n)) => *n,
                _ => 0,
            };
            done_counts.insert(style.to_string(), count);
        }
    }

    // Pipeline to group by style for practicing (is_sent = 0)
    let practicing_pipeline = vec![
        doc! { "$match": { "is_sent": 0 }},
        doc! { "$unwind": "$style" },
        doc! { "$group": { "_id": "$style", "count": { "$sum": 1 }}}
    ];

    let mut practicing_cursor = collection.aggregate(practicing_pipeline, None).await.map_err(|e| e.to_string())?;
    let mut practicing_counts = std::collections::HashMap::new();
    while let Some(doc) = practicing_cursor.try_next().await.map_err(|e| e.to_string())? {
        if let Some(style) = doc.get_str("_id").ok() {
            let count = match doc.get("count") {
                Some(bson::Bson::Int32(n)) => *n as i64,
                Some(bson::Bson::Int64(n)) => *n,
                _ => 0,
            };
            practicing_counts.insert(style.to_string(), count);
        }
    }

    // Build summary list
    let mut summary = Vec::new();
    for style in done_counts.keys().chain(practicing_counts.keys()) {
        let done = *done_counts.get(style).unwrap_or(&0);
        let practicing = *practicing_counts.get(style).unwrap_or(&0);
        summary.push((style.clone(), done, practicing));
    }

    Ok(summary)
}

// Uploads image data to Cloudinary and returns the secure_url.
#[tauri::command]
async fn upload_image(image_data: Vec<u8>, image_name: String) -> Result<String, String> { // image_data: A vector of bytes (Vec<u8>) representing the raw image data.
    let client = reqwest::Client::new();
    let cloud_name = "du9hsgxds"; // Replace with your Cloudinary cloud name
    let upload_preset = "shafaedyn"; // Replace with your upload preset

    // Createthe image part.
    let part = reqwest::multipart::Part::bytes(image_data) // Converts the raw image_data (a byte vector) into a multipart form part for file uploads.
        .file_name(image_name); // Specifies the image name for the uploaded file.

    // Create the multipart form.
    let form = reqwest::multipart::Form::new() // Creates a new multipart form object.
        .part("file", part) // Adds the image data to the form under the "file" field (required by Cloudinary).
        .text("upload_preset", upload_preset.to_string()); // Adds the upload preset to the form as a text field (Cloudinary requires an upload_preset unless you use authenticated uploads).

    // Send the upload request.
    let res = client
        .post(format!("https://api.cloudinary.com/v1_1/{}/upload", cloud_name))
        .multipart(form) // Attaches the multipart form containing the image data and preset.
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    // Handle successful upload.
    if res.status().is_success() { // Checks if the HTTP response indicates success (status code 2xx).
        let response_body: Value = res.json().await.map_err(|e| format!("Failed to parse JSON: {}", e))?; // res.json().await: Parses the response body as JSON asynchronously.
        // Extract the secure URL.
        if let Some(secure_url) = response_body.get("secure_url").and_then(Value::as_str) { // response_body.get("secure_url"): Retrieves the secure_url field from the JSON response.
            Ok(secure_url.to_string()) // Returns the secure URL if found.
        } else {
            Err("`secure_url` not found in Cloudinary response".to_string())
        }
    // Handle upload failure.
    } else {
        let status = res.status();
        let error_body = res.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
        Err(format!("Cloudinary upload failed (Status: {}): {}", status, error_body))
    }
}

#[tauri::command]
async fn get_project_by_id(state: State<'_, Arc<Mutex<DatabaseHelper>>>, id: String) -> Result<Option<Project>, String> {
    let db = state.lock().await;

    // Convert the string ID to ObjectId
    let object_id = ObjectId::parse_str(&id).map_err(|e| format!("Invalid ObjectId: {}", e))?;

    match db.get_project_by_id(&object_id).await {
        Ok(Some(project)) => Ok(Some(project)),
        Ok(none) => Ok(none),
        Err(err) => Err(format!("Error fetching project: {}", err)),
    }
}

#[tauri::command]
async fn create_account(email: String, password: String, db: State<'_, Arc<Mutex<DatabaseHelper>>>) -> Result<String, String> {
  // Lock the Mutex asynchronously
  let db = db.lock().await;
  db.create_account(&email, &password)
    .await
    .map(|oid| oid.to_hex())
    .map_err(|e| format!("Error creating account: {:?}", e))
}

#[tauri::command]
async fn login(email: String, password: String, db: State<'_, Arc<Mutex<DatabaseHelper>>>) -> Result<String, String> {
  // Lock the Mutex asynchronously
  let db = db.lock().await;
  db.login(&email, &password)
    .await
    .map_err(|e| format!("Error logging in: {:?}", e))
}