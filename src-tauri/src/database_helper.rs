// src-tauri/src/database_helper.rs

// Initialises the MongoDB database


// IMPORTS
// Serde for JSON serialization
use serde::{Deserialize, Serialize};

// MongoDB & BSON
use mongodb::{Client, bson::{self, doc}, Collection};
use bson::oid::ObjectId;

// Date handling
use chrono::Utc;

// Error types
use mongodb::error::Error;

// Secure password hashing & verifying
use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::{PasswordHash, SaltString, rand_core::OsRng}};

// JWT tokens
use jsonwebtoken::{encode, EncodingKey, Header};

// DATA STRUCTS

// Stores a coordinate marker on the image
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")] // Ensures field names in MongoDB match JSON camelCase
pub struct Coordinate {
    pub lat: f64,
    pub lng: f64,
    pub note: Vec<String>,
}

// Represents a bouldering project
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>, // MongoDB uses _id
    pub account_id: ObjectId,  // 🔥 foreign key to Account
    pub date_time: i64, // UNIX timestamp
    pub sent_date: Option<i64>, // Optional UNIX timestamp
    pub image_path: String,
    pub is_sent: i32, // Use 1 or 0 to match the JS client
    pub attempts: i32,
    pub grade: String,
    pub is_active: i32,
    pub coordinates: Vec<Coordinate>,
    pub style: Option<Vec<String>>,
    pub holds: Option<Vec<String>>,
}

// Normalization to ensure sent_date is only set when is_sent = 1
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

// Represents a JWT claim
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (usually the user identifier)
    exp: usize,  // Expiration time
}

// User accounts stored in the DB
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>, // MongoDB's unique ID
    pub email: String,         // User email (unique)
    pub hashed_password: String, // Securely stored password
    pub created_at: chrono::DateTime<chrono::Utc>, // Timestamp
}

// DATABASE HELPER
// Wraps a MongoDB client to share across calls
pub struct DatabaseHelper {
    client: Client, // Holds a MongoDB client to interact with the database.
}

// Implement all the logic on it
impl DatabaseHelper {
    // NEW INSTANCE
    // Connect to MongoDB, do a test ping
    pub async fn new() -> Result<Self, mongodb::error::Error> { // Connect to MongoDB.
        let mongo_uri = "***REMOVED***vness:8dSKcqijM7aVUH2V@hooked.1zbi6.mongodb.net/?retryWrites=true&w=majority&appName=Hooked"; // A string containing the MongoDB connection URI.
        let client = Client::with_uri_str(mongo_uri).await?; // Asynchronously connects to MongoDB using the URI.

        client.database("admin").run_command(doc! {"ping": 1}, None).await?; // client.database("admin"): Selects the admin database. .run_command(doc! {"ping": 1}, None).await?: Executes a ping command to check the database connection.
        Ok(DatabaseHelper { client }) // Returns an instance of DatabaseHelper with the connected client.
    }

    // PROJECTS

    // Fetch a single project by its ObjectId
    pub async fn get_project_by_id(&self, id: &ObjectId) -> Result<Option<Project>, mongodb::error::Error> {
        let database = self.client.database("hooked_db");
        let collection: Collection<Project> = database.collection("projects");

        let filter = doc! { "_id": id };
        let project = collection.find_one(filter, None).await?;

        Ok(project)
    }

    // ACCOUNTS

    // Create a new user account (register)
    pub async fn create_account(&self, email: &str, password: &str) -> Result<ObjectId, Error> {
        let database = self.client.database("hooked_db");
        let collection: Collection<Account> = database.collection("accounts");

        // Hash the password securely
        let salt = SaltString::generate(&mut OsRng); // Generates a secure random salt. (A salt is a random string of data that is generated each time a user creates (or changes) their password.It is then combined (concatenated) with the user's password before hashing.)
        let argon2 = Argon2::default(); // Uses secure default Argon2 parameters
        let hashed_password = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|_| Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Hashing error")))?
            .to_string();

        // Insert into database
        let new_account = Account {
            _id: None,
            email: email.to_string(),
            hashed_password,
            created_at: Utc::now(),
        };

        let insert_result = collection.insert_one(new_account, None).await?;
        Ok(insert_result.inserted_id.as_object_id().unwrap()) // Return inserted ID
    }

    // LOGIN
    // Verifies password + returns a JWT token
    pub async fn login(&self, email: &str, password: &str) -> Result<(String, ObjectId), Error> {
        let database = self.client.database("hooked_db");
        let collection: Collection<Account> = database.collection("accounts");

        let filter = doc! { "email": email };
        let user = collection.find_one(filter, None).await?;

        if let Some(user) = user {
            let parsed_hash = PasswordHash::new(&user.hashed_password).unwrap();
            let argon2 = Argon2::default();

            if argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok() {
                // Password correct. Generate JWT Token
                let expiration = Utc::now().timestamp() as usize + 3600; // 1-hour expiry
                let claims = Claims { sub: email.to_string(), exp: expiration };
                let secret = "my_secret_key"; // WARNING: should be ENV VAR in production!

                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
                    .map_err(|_| Error::from(std::io::Error::new(std::io::ErrorKind::Other, "JWT Error")))?;

                let account_id = user._id.ok_or_else(|| 
                    Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Account ID missing"))
                )?;

                return Ok((token, account_id));
            }
        }

        Err(Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Invalid credentials")))
    }
}