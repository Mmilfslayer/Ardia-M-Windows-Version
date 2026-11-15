use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::aead::generic_array::GenericArray;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use windows::Win32::Security::Cryptography::{
    CryptProtectData, CryptUnprotectData, CRYPTPROTECT_UI_FORBIDDEN,
};
use windows::Win32::Foundation::{LocalFree, HLOCAL};
use windows::core::Result as WinResult;

use crate::error::CoreResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PasswordId(pub Uuid);

impl PasswordId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for PasswordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub id: PasswordId,
    pub origin_url: String,
    pub action_url: String,
    pub username: String,
    pub password: String, // Encrypted
    pub username_element: Option<String>,
    pub password_element: Option<String>,
    pub date_created: DateTime<Utc>,
    pub date_last_used: Option<DateTime<Utc>>,
    pub times_used: u64,
    pub profile_id: String,
    pub is_federated: bool,
    pub federation_origin: Option<String>,
    pub display_name: Option<String>,
    pub icon_url: Option<String>,
}

pub struct PasswordManager {
    conn: Arc<RwLock<Connection>>,
    db_path: PathBuf,
}

impl PasswordManager {
    pub async fn new() -> CoreResult<Self> {
        let db_path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Ardia")
            .join("passwords.db");

        std::fs::create_dir_all(db_path.parent().unwrap())?;

        let conn = Connection::open(&db_path)?;

        // Create tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS passwords (
                id TEXT PRIMARY KEY,
                origin_url TEXT NOT NULL,
                action_url TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL, -- Encrypted with DPAPI
                username_element TEXT,
                password_element TEXT,
                date_created TEXT NOT NULL,
                date_last_used TEXT,
                times_used INTEGER NOT NULL DEFAULT 0,
                profile_id TEXT NOT NULL,
                is_federated INTEGER NOT NULL DEFAULT 0,
                federation_origin TEXT,
                display_name TEXT,
                icon_url TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_passwords_origin ON passwords(origin_url)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_passwords_profile ON passwords(profile_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_passwords_username ON passwords(username)",
            [],
        )?;

        Ok(Self {
            conn: Arc::new(RwLock::new(conn)),
            db_path,
        })
    }

    /// Encrypt data using Windows DPAPI (user-level protection)
    fn encrypt_dpapi(data: &[u8]) -> CoreResult<Vec<u8>> {
        let data_blob = windows::Win32::Security::Cryptography::DATA_BLOB {
            cbData: data.len() as u32,
            pbData: data.as_ptr() as *mut u8,
        };

        let mut encrypted_blob = windows::Win32::Security::Cryptography::DATA_BLOB::default();

        // Use CRYPTPROTECT_UI_FORBIDDEN to prevent UI prompts
        let entropy_blob = windows::Win32::Security::Cryptography::DATA_BLOB::default();

        unsafe {
            CryptProtectData(
                &data_blob,
                None,
                Some(&entropy_blob),
                None,
                None,
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut encrypted_blob,
            )?;
        }

        let result = unsafe {
            std::slice::from_raw_parts(encrypted_blob.pbData, encrypted_blob.cbData as usize).to_vec()
        };

        // Free the allocated memory
        unsafe {
            if !encrypted_blob.pbData.is_null() {
                LocalFree(HLOCAL(encrypted_blob.pbData as _));
            }
        }

        Ok(result)
    }

    /// Decrypt data using Windows DPAPI
    fn decrypt_dpapi(encrypted_data: &[u8]) -> CoreResult<Vec<u8>> {
        let encrypted_blob = windows::Win32::Security::Cryptography::DATA_BLOB {
            cbData: encrypted_data.len() as u32,
            pbData: encrypted_data.as_ptr() as *mut u8,
        };

        let mut decrypted_blob = windows::Win32::Security::Cryptography::DATA_BLOB::default();
        let entropy_blob = windows::Win32::Security::Cryptography::DATA_BLOB::default();

        unsafe {
            CryptUnprotectData(
                &encrypted_blob,
                None,
                Some(&entropy_blob),
                None,
                None,
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut decrypted_blob,
            )?;
        }

        let result = unsafe {
            std::slice::from_raw_parts(decrypted_blob.pbData, decrypted_blob.cbData as usize).to_vec()
        };

        // Free the allocated memory
        unsafe {
            if !decrypted_blob.pbData.is_null() {
                LocalFree(HLOCAL(decrypted_blob.pbData as _));
            }
        }

        Ok(result)
    }

    /// Encrypt password for storage
    fn encrypt_password(&self, password: &str) -> CoreResult<String> {
        let encrypted = Self::encrypt_dpapi(password.as_bytes())?;
        Ok(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, encrypted))
    }

    /// Decrypt password from storage
    fn decrypt_password(&self, encrypted: &str) -> CoreResult<String> {
        let encrypted_bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encrypted)?;
        let decrypted = Self::decrypt_dpapi(&encrypted_bytes)?;
        Ok(String::from_utf8(decrypted)?)
    }

    pub async fn add_password(
        &self,
        origin_url: String,
        action_url: String,
        username: String,
        password: String,
        username_element: Option<String>,
        password_element: Option<String>,
        profile_id: String,
        is_federated: bool,
        federation_origin: Option<String>,
        display_name: Option<String>,
        icon_url: Option<String>,
    ) -> CoreResult<PasswordId> {
        let conn = self.conn.read().await;

        let encrypted_password = self.encrypt_password(&password)?;

        let id = PasswordId::new();
        let now = Utc::now();

        conn.execute(
            "INSERT INTO passwords (id, origin_url, action_url, username, password, username_element, password_element,
             date_created, profile_id, is_federated, federation_origin, display_name, icon_url)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                id.to_string(),
                &origin_url,
                &action_url,
                &username,
                &encrypted_password,
                username_element.as_deref(),
                password_element.as_deref(),
                now.to_rfc3339(),
                &profile_id,
                is_federated as i32,
                federation_origin.as_deref(),
                display_name.as_deref(),
                icon_url.as_deref(),
            ],
        )?;

        Ok(id)
    }

    pub async fn update_password(&self, id: PasswordId, password: String) -> CoreResult<()> {
        let conn = self.conn.read().await;
        let encrypted_password = self.encrypt_password(&password)?;

        conn.execute(
            "UPDATE passwords SET password = ?1, date_last_used = ?2, times_used = times_used + 1 WHERE id = ?3",
            params![&encrypted_password, Utc::now().to_rfc3339(), id.to_string()],
        )?;

        Ok(())
    }

    pub async fn get_password(&self, id: PasswordId) -> CoreResult<Option<PasswordEntry>> {
        let conn = self.conn.read().await;

        let entry = conn
            .query_row(
                "SELECT id, origin_url, action_url, username, password, username_element, password_element,
                 date_created, date_last_used, times_used, profile_id, is_federated, federation_origin, display_name, icon_url
                 FROM passwords WHERE id = ?1",
                params![id.to_string()],
                |row| {
                    let encrypted_password: String = row.get(4)?;
                    let password = self.decrypt_password(&encrypted_password).unwrap_or_default();

                    Ok(PasswordEntry {
                        id: PasswordId(Uuid::parse_str(&row.get::<_, String>(0)?).unwrap()),
                        origin_url: row.get(1)?,
                        action_url: row.get(2)?,
                        username: row.get(3)?,
                        password,
                        username_element: row.get(5)?,
                        password_element: row.get(6)?,
                        date_created: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?).unwrap().with_timezone(&Utc),
                        date_last_used: row.get::<_, Option<String>>(8)?.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                        times_used: row.get(9)?,
                        profile_id: row.get(10)?,
                        is_federated: row.get::<_, i32>(11)? != 0,
                        federation_origin: row.get(12)?,
                        display_name: row.get(13)?,
                        icon_url: row.get(14)?,
                    })
                },
            )
            .optional()?;

        Ok(entry)
    }

    pub async fn find_passwords(
        &self,
        origin_url: &str,
        profile_id: &str,
    ) -> CoreResult<Vec<PasswordEntry>> {
        let conn = self.conn.read().await;

        let mut stmt = conn.prepare(
            "SELECT id, origin_url, action_url, username, password, username_element, password_element,
             date_created, date_last_used, times_used, profile_id, is_federated, federation_origin, display_name, icon_url
             FROM passwords
             WHERE origin_url LIKE ?1 AND profile_id = ?2
             ORDER BY times_used DESC, date_last_used DESC",
        )?;

        let pattern = format!("{}%", origin_url);

        let entries = stmt
            .query_map(params![pattern, profile_id], |row| {
                let encrypted_password: String = row.get(4)?;
                let password = self.decrypt_password(&encrypted_password).unwrap_or_default();

                Ok(PasswordEntry {
                    id: PasswordId(Uuid::parse_str(&row.get::<_, String>(0)?).unwrap()),
                    origin_url: row.get(1)?,
                    action_url: row.get(2)?,
                    username: row.get(3)?,
                    password,
                    username_element: row.get(5)?,
                    password_element: row.get(6)?,
                    date_created: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?).unwrap().with_timezone(&Utc),
                    date_last_used: row.get::<_, Option<String>>(8)?.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                    times_used: row.get(9)?,
                    profile_id: row.get(10)?,
                    is_federated: row.get::<_, i32>(11)? != 0,
                    federation_origin: row.get(12)?,
                    display_name: row.get(13)?,
                    icon_url: row.get(14)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    pub async fn find_all_passwords(&self, profile_id: &str) -> CoreResult<Vec<PasswordEntry>> {
        let conn = self.conn.read().await;

        let mut stmt = conn.prepare(
            "SELECT id, origin_url, action_url, username, password, username_element, password_element,
             date_created, date_last_used, times_used, profile_id, is_federated, federation_origin, display_name, icon_url
             FROM passwords
             WHERE profile_id = ?1
             ORDER BY origin_url, username",
        )?;

        let entries = stmt
            .query_map(params![profile_id], |row| {
                let encrypted_password: String = row.get(4)?;
                let password = self.decrypt_password(&encrypted_password).unwrap_or_default();

                Ok(PasswordEntry {
                    id: PasswordId(Uuid::parse_str(&row.get::<_, String>(0)?).unwrap()),
                    origin_url: row.get(1)?,
                    action_url: row.get(2)?,
                    username: row.get(3)?,
                    password,
                    username_element: row.get(5)?,
                    password_element: row.get(6)?,
                    date_created: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?).unwrap().with_timezone(&Utc),
                    date_last_used: row.get::<_, Option<String>>(8)?.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                    times_used: row.get(9)?,
                    profile_id: row.get(10)?,
                    is_federated: row.get::<_, i32>(11)? != 0,
                    federation_origin: row.get(12)?,
                    display_name: row.get(13)?,
                    icon_url: row.get(14)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    pub async fn delete_password(&self, id: PasswordId) -> CoreResult<()> {
        let conn = self.conn.read().await;
        conn.execute("DELETE FROM passwords WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }

    pub async fn delete_all_passwords(&self, profile_id: &str) -> CoreResult<()> {
        let conn = self.conn.read().await;
        conn.execute("DELETE FROM passwords WHERE profile_id = ?1", params![profile_id])?;
        Ok(())
    }

    pub async fn update_password_metadata(
        &self,
        id: PasswordId,
        username: Option<String>,
        display_name: Option<String>,
    ) -> CoreResult<()> {
        let conn = self.conn.read().await;

        let mut updates = Vec::new();
        let mut params_vec = Vec::new();

        if let Some(username) = username {
            updates.push("username = ?");
            params_vec.push(username);
        }
        if let Some(display_name) = display_name {
            updates.push("display_name = ?");
            params_vec.push(display_name);
        }

        if !updates.is_empty() {
            params_vec.push(id.to_string());
            let query = format!("UPDATE passwords SET {} WHERE id = ?", updates.join(", "));
            let params: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
            conn.execute(&query, rusqlite::params_from_iter(params))?;
        }

        Ok(())
    }

    pub async fn record_password_use(&self, id: PasswordId) -> CoreResult<()> {
        let conn = self.conn.read().await;
        conn.execute(
            "UPDATE passwords SET date_last_used = ?1, times_used = times_used + 1 WHERE id = ?2",
            params![Utc::now().to_rfc3339(), id.to_string()],
        )?;
        Ok(())
    }

    pub async fn export_passwords(&self, profile_id: &str) -> CoreResult<String> {
        let entries = self.find_all_passwords(profile_id).await?;
        // Export without passwords for security
        let export_entries: Vec<_> = entries.into_iter().map(|mut e| {
            e.password = "***".to_string();
            e
        }).collect();
        Ok(serde_json::to_string_pretty(&export_entries)?)
    }

    pub async fn save(&self) -> CoreResult<()> {
        let conn = self.conn.read().await;
        conn.execute("PRAGMA wal_checkpoint(TRUNCATE)", [])?;
        Ok(())
    }

    pub fn get_db_path(&self) -> &PathBuf {
        &self.db_path
    }
}

impl Default for PasswordManager {
    fn default() -> Self {
        Self {
            conn: Arc::new(RwLock::new(Connection::open_in_memory().unwrap())),
            db_path: PathBuf::from("."),
        }
    }
}