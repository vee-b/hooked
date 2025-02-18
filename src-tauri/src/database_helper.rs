// src-tauri/src/database_helper.rs

// Initialises the MongoDB database

use serde::{Deserialize, Serialize};
use mongodb::{Client, bson::{self, doc}, Collection};
//use futures_util::TryStreamExt;
use bson::oid::ObjectId;

// Define the Project struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>, // Use ObjectId for MongoDB's _id
    pub date_time: i64,
    pub image_path: String,
    pub is_sent: i32,
    pub attempts: i32,
    pub grade: String,
    pub is_active: i32,
}

// Define the DatabaseHelper struct.
pub struct DatabaseHelper {
    client: Client, // Holds a MongoDB client to interact with the database.
}

// Implement DatabaseHelper.
impl DatabaseHelper {
    pub async fn new() -> Result<Self, mongodb::error::Error> { // Connect to MongoDB.
        let mongo_uri = "***REMOVED***vness:8dSKcqijM7aVUH2V@hooked.1zbi6.mongodb.net/?retryWrites=true&w=majority&appName=Hooked"; // A string containing the MongoDB connection URI.
        let client = Client::with_uri_str(mongo_uri).await?; // Asynchronously connects to MongoDB using the URI.
        client.database("admin").run_command(doc! {"ping": 1}, None).await?; // client.database("admin"): Selects the admin database. .run_command(doc! {"ping": 1}, None).await?: Executes a ping command to check the database connection.
        Ok(DatabaseHelper { client }) // Returns an instance of DatabaseHelper with the connected client.
    }
}