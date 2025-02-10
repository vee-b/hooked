use serde::{Deserialize, Serialize};
use mongodb::{Client, bson::{self, doc}, Collection};
use futures_util::TryStreamExt;
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

    fn get_collection(&self) -> Collection<Project> {
        let db = self.client.database("hooked_db");
        db.collection::<Project>("projects")
    }

    pub async fn insert_project(&self, project: Project) -> Result<(), mongodb::error::Error> {
        let collection = self.get_collection();
        collection.insert_one(project, None).await?;
        Ok(())
    }

    pub async fn get_all_projects(&self) -> Result<Vec<Project>, mongodb::error::Error> {
        let collection = self.get_collection();
        let cursor = collection.find(None, None).await?;
        let projects: Vec<Project> = cursor.try_collect().await?;  // More efficient collection
        Ok(projects)
    }

    pub async fn delete_project(&self, _id: &str) -> Result<(), String> {
        let collection = self.get_collection();
        let object_id = ObjectId::parse_str(_id).map_err(|e| e.to_string())?;
        let filter = doc! {"_id": object_id};
        collection.delete_one(filter, None).await.map_err(|e| e.to_string())?;
        Ok(())
    }


    pub async fn get_sends_count(&self) -> Result<i64, String> { // Removed unnecessary client argument
        let collection = self.get_collection();
        let count = collection.count_documents(None, None).await.map_err(|e| e.to_string())?;
        Ok(count.try_into().unwrap_or(i64::MAX)) // Handle potential overflow
    }

    pub async fn get_active_projects(&self) -> Result<Vec<Project>, mongodb::error::Error> {
        let collection = self.get_collection();
        let filter = doc! { "is_active": 1 };
        let cursor = collection.find(filter, None).await?;
        let projects: Vec<Project> = cursor.try_collect().await?;
        Ok(projects)
    }
}