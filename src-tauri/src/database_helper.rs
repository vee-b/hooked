// For SQLite DB

// use serde::{Deserialize, Serialize};
// use sqlx::{FromRow, Row, SqlitePool}; // Rust SQL toolkit for asynchronous database interactions. SqlitePool manages connections to the SQLite database, and Row is used to access results from a query. // Serialization/deserialization framework. Serialize and Deserialize are used to convert Rust structs to and from JSON or other formats.

// // Define the Project struct
// #[derive(FromRow, Debug, Serialize, Deserialize)]
// pub struct Project {
//     pub id: Option<String>,         // Keep as String, adjust if necessary
//     pub date_time: i64,     // Use i64 for date/time representation
//     pub image_path: String, // Image path
//     pub is_sent: i32,       // Use i32 for the is_sent field
//     pub attempts: i32,      // Number of attempts
//     pub grade: String,      // Project grade
//     pub is_active: i32,
// }

// // Database struct (Manages the SQLite connection pool (SqlitePool), which allows asynchronous queries on the database)
// pub struct DatabaseHelper {
//     pool: SqlitePool,
// }

// impl DatabaseHelper {
//     // Create a new instance of DatabaseHelper
//     pub async fn new() -> Result<Self, sqlx::Error> {
//         let db_path = Self::get_database_path()?; // Retrieves or creates the path to the SQLite file.
//         let pool = SqlitePool::connect(&db_path).await?; // Opens a connection to the SQLite database using the db_path.
//         Self::initialize(&pool).await?; // Ensures the database is properly initialized by creating the necessary tables.
//         Ok(DatabaseHelper { pool })
//     }

//     // Constructs the file path for the SQLite database.
//     fn get_database_path() -> Result<String, std::io::Error> {
//         let project_dir = std::env::current_dir()?;
//         let db_dir = project_dir.join("database");

//         // If the "database" directory or the project.db file doesn't exist, it creates it.
//         if !db_dir.exists() {
//             std::fs::create_dir(&db_dir)?;
//         }

//         let db_path = db_dir.join("project.db");
//         println!("Database path: {}", db_path.display()); // Log the database path

//         if !db_path.exists() {
//             std::fs::File::create(&db_path)?;
//         }

//         // Returns the file path in a format SQLite understands (sqlite:path_to_db).
//         Ok(format!("sqlite:{}", db_path.to_string_lossy()))
//     }

//     // Initialize the database and create the table
//     async fn initialize(pool: &SqlitePool) -> Result<(), sqlx::Error> {
//         // Drop the existing table to reset the schema (use only in development)
//         // sqlx::query("DROP TABLE IF EXISTS projectTable")
//         // .execute(pool)
//         // .await?;

//         sqlx::query(
//             r#"
//             CREATE TABLE IF NOT EXISTS projectTable (
//                 id TEXT PRIMARY KEY,
//                 date_time INTEGER,
//                 image_path TEXT,
//                 is_sent INTEGER,
//                 attempts INTEGER,
//                 grade TEXT,
//                 is_active INTEGER NOT NULL DEFAULT 1
//             )
//             "#,
//         )
//         .execute(pool)
//         .await?;
//         Ok(())
//     }

//     // Insert a project into the database
//     pub async fn insert_project(&self, project: Project) -> Result<(), sqlx::Error> {
//         let result = sqlx::query(
//             r#"
//             INSERT INTO projectTable (id, date_time, image_path, is_sent, attempts, grade, is_active)
//             VALUES (?, ?, ?, ?, ?, ?, ?)
//             "#,
//         )
//         .bind(project.id) // Ensure correct binding
//         .bind(project.date_time) // Correct date_time binding
//         .bind(project.image_path) // Bind image_path
//         .bind(project.is_sent) // Bind is_sent
//         .bind(project.attempts) // Bind attempts
//         .bind(project.grade) // Bind grade
//         .bind(project.is_active) // Bind active
//         .execute(&self.pool) // Execute against the pool
//         .await; // Handle any errors

//         if let Err(e) = result {
//             println!("Error during insert: {:?}", e);
//             return Err(e);
//         }
//         Ok(())
//     }

//     // Fetch all projects from the database
//     pub async fn get_all_projects(&self) -> Result<Vec<Project>, sqlx::Error> {
//         let rows = sqlx::query(
//             r#"
//             SELECT id, date_time, image_path, is_sent, attempts, grade, is_active
//             FROM projectTable
//             ORDER BY date_time ASC
//             "#,
//         )
//         .fetch_all(&self.pool)
//         .await?;

//         let projects = rows
//             .iter()
//             .map(|row| Project {
//                 id: row.get("id"),
//                 date_time: row.get("date_time"),
//                 image_path: row.get("image_path"),
//                 is_sent: row.get("is_sent"),
//                 attempts: row.get("attempts"),
//                 grade: row.get("grade"),
//                 is_active: row.get("is_active"),
//             })
//             .collect();

//         Ok(projects)
//     }

//     // Update a project in the database
//     // pub async fn update_project(&self, project: Project) -> Result<(), sqlx::Error> {
//     //     sqlx::query(
//     //         r#"
//     //         UPDATE projectTable
//     //         SET date_time = ?, image_path = ?, is_sent = ?, attempts = ?, grade = ?, is_active = ?,
//     //         WHERE id = ?
//     //         "#,
//     //     )
//     //     .bind(project.date_time) // Update bindings as necessary
//     //     .bind(project.image_path)
//     //     .bind(project.is_sent)
//     //     .bind(project.attempts)
//     //     .bind(project.grade)
//     //     .bind(project.id)
//     //     .bind(project.is_active)
//     //     .execute(&self.pool)
//     //     .await?;
//     //     Ok(())
//     // }

//     // Delete a project from the database
//     pub async fn delete_project(&self, id: &str) -> Result<(), sqlx::Error> {
//         sqlx::query(
//             r#"
//             DELETE FROM projectTable
//             WHERE id = ?
//             "#,
//         )
//         .bind(id)
//         .execute(&self.pool)
//         .await?;
//         Ok(())
//     }

//     // Get the count of projects in the database
//     pub async fn get_sends_count(&self) -> Result<i64, sqlx::Error> {
//         let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM projectTable")
//             .fetch_one(&self.pool)
//             .await?;
//         Ok(count.0)
//     }

//     // pub async fn get_active_projects(&self) -> Result<Vec<Project>, sqlx::Error> {
//     //     let rows = sqlx::query_as::<_, Project>(
//     //         "SELECT id, date_time, image_path, is_sent, attempts, grade, is_active
//     //          FROM projectTable
//     //          WHERE active = 1"
//     //     )
//     //     .fetch_all(&self.pool)
//     //     .await?;

//     pub async fn get_active_projects(&self) -> Result<Vec<Project>, sqlx::Error> {
//         let query = "SELECT * FROM projectTable WHERE is_active = 1";
//         let projects = sqlx::query_as::<_, Project>(query)
//             .fetch_all(&self.pool)
//             .await?;
//         Ok(projects)
//     }
// }
























// For MongoDB

use serde::{Deserialize, Serialize};
use mongodb::{Client, bson::{self, doc}, Collection};
//use futures_util::StreamExt;
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
