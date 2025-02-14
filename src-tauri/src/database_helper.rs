// src-tauri/src/database_helper.rs

// Initialises the MongoDB database

use serde::{Deserialize, Serialize};
use mongodb::{Client, bson::{self, doc}, Collection};
//use futures_util::TryStreamExt;
use bson::oid::ObjectId;

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

pub struct DatabaseHelper {
    client: Client,
}

impl DatabaseHelper {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let mongo_uri = "***REMOVED***vness:8dSKcqijM7aVUH2V@hooked.1zbi6.mongodb.net/?retryWrites=true&w=majority&appName=Hooked"; // Replace with your actual URI
        let client = Client::with_uri_str(mongo_uri).await?;
        client.database("admin").run_command(doc! {"ping": 1}, None).await?;
        Ok(DatabaseHelper { client })
    }

    // fn get_collection(&self) -> Collection<Project> {
    //     let db = self.client.database("hooked_db");
    //     db.collection::<Project>("projects")
    // }
}