use sqlx::{SqlitePool, Row}; // Rust SQL toolkit for asynchronous database interactions. SqlitePool manages connections to the SQLite database, and Row is used to access results from a query.
use serde::{Deserialize, Serialize}; // Serialization/deserialization framework. Serialize and Deserialize are used to convert Rust structs to and from JSON or other formats.

// Define the Project struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,              // Keep as String, adjust if necessary
    pub date_time: i64,          // Use i64 for date/time representation
    pub image_path: String,       // Image path
    pub is_sent: i32,            // Use i32 for the is_sent field
    pub attempts: i32,           // Number of attempts
    pub grade: String,           // Project grade
}

// Database struct (Manages the SQLite connection pool (SqlitePool), which allows asynchronous queries on the database)
pub struct DatabaseHelper {
    pool: SqlitePool,
}

impl DatabaseHelper {
    // Create a new instance of DatabaseHelper
    pub async fn new() -> Result<Self, sqlx::Error> {
        let db_path = Self::get_database_path()?; // Retrieves or creates the path to the SQLite file.
        let pool = SqlitePool::connect(&db_path).await?; // Opens a connection to the SQLite database using the db_path.
        Self::initialize(&pool).await?; // Ensures the database is properly initialized by creating the necessary tables.
        Ok(DatabaseHelper { pool })
    }

    // Constructs the file path for the SQLite database.
    fn get_database_path() -> Result<String, std::io::Error> {
        let project_dir = std::env::current_dir()?;
        let db_dir = project_dir.join("database");
    
        // If the "database" directory or the project.db file doesn't exist, it creates it.
        if !db_dir.exists() {
            std::fs::create_dir(&db_dir)?;
        }
    
        let db_path = db_dir.join("project.db");
        println!("Database path: {}", db_path.display()); // Log the database path
    
        if !db_path.exists() {
            std::fs::File::create(&db_path)?;
        }
    
        // Returns the file path in a format SQLite understands (sqlite:path_to_db).
        Ok(format!("sqlite:{}", db_path.to_string_lossy()))
    }
    
    // Initialize the database and create the table
    async fn initialize(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS projectTable (
                id TEXT PRIMARY KEY,
                date_time INTEGER,
                image_path TEXT,
                is_sent INTEGER,
                attempts INTEGER,
                grade TEXT
            )
            "#,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    // Insert a project into the database
    pub async fn insert_project(&self, project: Project) -> Result<(), sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO projectTable (id, date_time, image_path, is_sent, attempts, grade)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(project.id) // Ensure correct binding
        .bind(project.date_time) // Correct date_time binding
        .bind(project.image_path) // Bind image_path
        .bind(project.is_sent) // Bind is_sent
        .bind(project.attempts) // Bind attempts
        .bind(project.grade) // Bind grade
        .execute(&self.pool) // Execute against the pool
        .await; // Handle any errors

        if let Err(e) = result {
            println!("Error during insert: {:?}", e);
            return Err(e);
        }
        Ok(())
    }

    // Fetch all projects from the database
    pub async fn get_all_projects(&self) -> Result<Vec<Project>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, dateTime, imagePath, isSent, attempts, grade
            FROM projectTable
            ORDER BY dateTime ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let projects = rows.iter().map(|row| Project {
            id: row.get("id"),
            date_time: row.get("dateTime"),
            image_path: row.get("imagePath"),
            is_sent: row.get("isSent"),
            attempts: row.get("attempts"),
            grade: row.get("grade"),
        }).collect();

        Ok(projects)
    }

    // Update a project in the database
    pub async fn update_project(&self, project: Project) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE projectTable
            SET dateTime = ?, imagePath = ?, isSent = ?, attempts = ?, grade = ?
            WHERE id = ?
            "#,
        )
        .bind(project.date_time) // Update bindings as necessary
        .bind(project.image_path)
        .bind(project.is_sent)
        .bind(project.attempts)
        .bind(project.grade)
        .bind(project.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // Delete a project from the database
    pub async fn delete_project(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM projectTable
            WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // Get the count of projects in the database
    pub async fn get_sends_count(&self) -> Result<i64, sqlx::Error> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM projectTable")
            .fetch_one(&self.pool)
            .await?;
        Ok(count.0)
    }

    // Optionally, delete the database file (if needed)
    // pub async fn delete_database_file() -> Result<(), std::io::Error> {
    //     std::fs::remove_file("database/project.db")?; // Make sure the path is correct
    //     Ok(())
    // }
}



// //use sqlx::{Sqlite, SqlitePool};
// //use sqlx::sqlite::SqliteConnectOptions;
// //use std::path::PathBuf;
// //use sqlx::Row; // Add this import
// use sqlx::{SqlitePool, Row};
// //use dirs;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Project {
//     pub id: String,
//     pub date_time: i64,
//     pub project_name: String,
//     pub image_path: String,
//     pub is_sent: i32,
//     pub attempts: i32,
//     pub grade: String,
// }

// pub struct DatabaseHelper {
//     pool: SqlitePool,
// }

// impl DatabaseHelper {
//     // Create a new instance of DatabaseHelper
//     pub async fn new() -> Result<Self, sqlx::Error> {
//         let db_path = Self::get_database_path()?;
//         let pool = SqlitePool::connect(&db_path).await?;
//         Self::initialize(&pool).await?;
//         Ok(DatabaseHelper { pool })
//     }

//     fn get_database_path() -> Result<String, std::io::Error> {
//         // Get the path to the current working directory (your project's root)
//         let project_dir = std::env::current_dir()?;
        
//         // Create a subdirectory for the database file, like "database"
//         let db_dir = project_dir.join("database");
        
//         // Create the directory if it doesn't exist
//         if !db_dir.exists() {
//             std::fs::create_dir(&db_dir)?;
//         }
    
//         // Construct the full path to the database
//         let db_path = db_dir.join("project.db");
//         println!("Database path: {}", db_path.display()); // Log the database path
    
//         // Ensure the file exists (create it if it doesn't)
//         if !db_path.exists() {
//             std::fs::File::create(&db_path)?;
//         }
    
//         // Return the path with the "sqlite:" prefix for SqlitePool to handle
//         Ok(format!("sqlite:{}", db_path.to_string_lossy()))
//     }
    

//     // Initialize the database and create the table
//     async fn initialize(pool: &SqlitePool) -> Result<(), sqlx::Error> {
//         sqlx::query(
//             r#"
//             CREATE TABLE IF NOT EXISTS projectTable (
//                 id TEXT PRIMARY KEY,
//                 dateTime INTEGER,
//                 projectName TEXT,
//                 imagePath TEXT,
//                 isSent INTEGER,
//                 attempts INTEGER,
//                 grade TEXT
//             )
//             "#,
//         )
//         .execute(pool)
//         .await?;
//         Ok(())
//     }

//     // Insert a project into the database
//     pub async fn insert_project(&self, project: Project) -> Result<(), sqlx::Error> {
//         sqlx::query(
//             r#"
//             INSERT INTO projectTable (id, dateTime, projectName, imagePath, isSent, attempts, grade)
//             VALUES (?, ?, ?, ?, ?, ?, ?)
//             "#,
//         )
//         .bind(project.id)
//         .bind(project.date_time)
//         .bind(project.project_name)
//         .bind(project.image_path)
//         .bind(project.is_sent)
//         .bind(project.attempts)
//         .bind(project.grade)
//         .execute(&self.pool)
//         .await?;
//         Ok(())
//     }

//     // Fetch all projects from the database
//     pub async fn fetch_projects(&self) -> Result<Vec<Project>, sqlx::Error> {
//         let rows = sqlx::query(
//             r#"
//             SELECT id, dateTime, projectName, imagePath, isSent, attempts, grade
//             FROM projectTable
//             ORDER BY dateTime ASC
//             "#,
//         )
//         .fetch_all(&self.pool)
//         .await?;

//         let projects = rows.iter().map(|row| Project {
//             id: row.get("id"),
//             date_time: row.get("dateTime"),
//             project_name: row.get("projectName"),
//             image_path: row.get("imagePath"),
//             is_sent: row.get("isSent"),
//             attempts: row.get("attempts"),
//             grade: row.get("grade"),
//         }).collect();

//         Ok(projects)
//     }

//     // Update a project in the database
//     pub async fn update_project(&self, project: Project) -> Result<(), sqlx::Error> {
//         sqlx::query(
//             r#"
//             UPDATE projectTable
//             SET dateTime = ?, projectName = ?, imagePath = ?, isSent = ?, attempts = ?, grade = ?
//             WHERE id = ?
//             "#,
//         )
//         .bind(project.date_time)
//         .bind(project.project_name)
//         .bind(project.image_path)
//         .bind(project.is_sent)
//         .bind(project.attempts)
//         .bind(project.grade)
//         .bind(project.id)
//         .execute(&self.pool)
//         .await?;
//         Ok(())
//     }

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
//     pub async fn get_count(&self) -> Result<i64, sqlx::Error> {
//         let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM projectTable")
//             .fetch_one(&self.pool)
//             .await?;
//         Ok(count.0)
//     }

//     // Optionally, delete the database file (if needed)
//     pub async fn delete_database_file() -> Result<(), std::io::Error> {
//         std::fs::remove_file("project.db")?;
//         Ok(())
//     }
// }
