// src-tauri/src/database_helper.rs

// Initialises the MongoDB database

use serde::{Deserialize, Serialize};
use mongodb::{Client, bson::{self, doc}, Collection};
use bson::oid::ObjectId;
use chrono::Utc;
use mongodb::error::Error;
use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::{PasswordHash, SaltString, rand_core::OsRng}};
use jsonwebtoken::{encode, EncodingKey, Header};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")] // Ensures MongoDB field names match
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
    pub note: Vec<String>,
}

// Define the Project struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>, // Use ObjectId for MongoDB's _id
    pub date_time: i64,
    pub sent_date: Option<i64>,
    pub image_path: String,
    pub is_sent: i32,
    pub attempts: i32,
    pub grade: String,
    pub is_active: i32,
    pub coordinates: Vec<Coordinate>,
    pub style: Option<Vec<String>>,
}

impl Project {
    pub fn normalize(&mut self) {
        if self.is_sent != 0 {
            if self.sent_date.is_none() {
                self.sent_date = Some(Utc::now().timestamp());
            }
        } else {
            self.sent_date = None;
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (usually the user identifier)
    exp: usize,  // Expiration time
}

// Define the Account struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>, // MongoDB's unique ID
    pub email: String,         // User email (unique)
    pub hashed_password: String, // Securely stored password
    pub created_at: chrono::DateTime<chrono::Utc>, // Timestamp
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

    // Fetch a project by its ID
    pub async fn get_project_by_id(&self, id: &ObjectId) -> Result<Option<Project>, mongodb::error::Error> {
        let database = self.client.database("hooked_db");
        let collection: Collection<Project> = database.collection("projects");

        let filter = doc! { "_id": id };
        let project = collection.find_one(filter, None).await?;

        Ok(project)
    }

    // Register a new user
    pub async fn create_account(&self, email: &str, password: &str) -> Result<ObjectId, Error> {
        let database = self.client.database("hooked_db");
        let collection: Collection<Account> = database.collection("accounts");

        // Hash the password
        let salt = SaltString::generate(&mut OsRng); // ✅ Generates a secure random salt
        let argon2 = Argon2::default(); // ✅ Uses default Argon2 parameters
        let hashed_password = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|_| Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Hashing error")))?
            .to_string();

        // Create the account
        let new_account = Account {
            _id: None,
            email: email.to_string(),
            hashed_password,
            created_at: Utc::now(),
        };

        let insert_result = collection.insert_one(new_account, None).await?;
        Ok(insert_result.inserted_id.as_object_id().unwrap()) // Return inserted ID
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<String, Error> {
        let database = self.client.database("hooked_db");
        let collection: Collection<Account> = database.collection("accounts");

        let filter = doc! { "email": email };
        let user = collection.find_one(filter, None).await?;

        if let Some(user) = user {
            let parsed_hash = PasswordHash::new(&user.hashed_password).unwrap();
            let argon2 = Argon2::default();

            if argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok() {
                // ✅ Generate JWT Token
                let expiration = Utc::now().timestamp() as usize + 3600; // 1-hour expiry
                let claims = Claims { sub: email.to_string(), exp: expiration };
                let secret = "my_secret_key"; // Replace with env var!
                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
                    .map_err(|_| Error::from(std::io::Error::new(std::io::ErrorKind::Other, "JWT Error")))?;

                return Ok(token);
            }
        }

        Err(Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Invalid credentials")))
    }
}