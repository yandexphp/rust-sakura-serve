use tokio::fs::{File, OpenOptions, create_dir_all};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use std::path::Path;
use tokio::sync::Mutex;
use std::error::Error as StdError;
use log::info;
use uuid::Uuid;

use crate::models::user::User;

pub struct UserStore {
    pub users: Mutex<Vec<User>>,
    pub users_file_path: String,
}

impl UserStore {
    pub async fn new(users_file_path: String) -> Result<Self, Box<dyn StdError>> {
        let path = Path::new(&users_file_path);

        if let Some(parent) = path.parent() {
            create_dir_all(parent).await.expect("Failed to create directories for users.json file");
        }

        if !path.exists() {
            let mut file = File::create(path).await.expect("Failed to create users.json file");
            file.write_all(b"[]").await.expect("Failed to write empty array to file");
        }

        let file = File::open(path).await.expect("Failed to open users.json file");
        let mut reader = BufReader::new(file);
        let mut data = String::new();
        reader.read_to_string(&mut data).await.expect("Failed to read file");

        let users: Vec<User> = serde_json::from_str(&data)?;

        Ok(UserStore {
            users: Mutex::new(users),
            users_file_path,
        })
    }

    pub async fn save(&self) -> Result<(), Box<dyn StdError>> {
        info!("Saving users to file...");
    
        let users = self.users.lock().await;
        info!("Number of users in store: {}", users.len());
    
        for (i, user) in users.iter().enumerate() {
            match serde_json::to_string_pretty(user) {
                Ok(data) => info!("User {} serialized successfully: {}", i, data),
                Err(e) => info!("Failed to serialize user {}: {:?}", i, e),
            }
        }
    
        let data = serde_json::to_string_pretty(&*users)?;
        
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.users_file_path)
            .await
            .expect("Failed to open users.json file for writing");
    
        file.write_all(data.as_bytes()).await?;
        info!("Users successfully saved.");
        Ok(())
    }

    pub async fn add_user(&self, user: User) -> Result<(), Box<dyn StdError>> {
        info!("Locking users to add new user...");
        
        let mut users = self.users.lock().await;
        info!("User store locked. Adding user: {}", user.username);
        
        users.push(user);
        info!("User added.");
    
        drop(users);
    
        info!("Saving user data to file...");
        self.save().await?;
        
        info!("User saved successfully.");
        Ok(())
    }

    pub async fn find_user_by_username(&self, username: &str) -> Option<User> {
        let users = self.users.lock().await;
        let result = users.iter()
            .cloned()
            .find(|user| user.username == username);
    
        if result.is_some() {
            info!("User found for: {}", username);
        } else {
            info!("No user found for: {}", username);
        }
    
        result
    }

    pub async fn find_user_by_login_or_email(&self, login_or_email: &str) -> Option<User> {
        let users = self.users.lock().await;
        let result = users.iter()
            .cloned()
            .find(|user| user.login == login_or_email || user.email == login_or_email);
    
        if result.is_some() {
            info!("User found for: {}", login_or_email);
        } else {
            info!("No user found for: {}", login_or_email);
        }
    
        result
    }

    pub async fn find_user_by_id(&self, user_id: Uuid) -> Option<User> {
        let users = self.users.lock().await;
        users.iter().find(|u| u.id == user_id).cloned()
    }

    pub async fn update_user(&self, updated_user: User) -> bool {
        let mut users = self.users.lock().await;
        if let Some(pos) = users.iter().position(|u| u.id == updated_user.id) {
            users[pos] = updated_user;
            true
        } else {
            false
        }
    }
}
