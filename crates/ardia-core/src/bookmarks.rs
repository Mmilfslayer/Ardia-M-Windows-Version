use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::error::CoreResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BookmarkId(pub Uuid);

impl BookmarkId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for BookmarkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: BookmarkId,
    pub parent_id: Option<BookmarkId>,
    pub index: i32,
    pub title: String,
    pub url: Option<String>, // None for folders
    pub date_added: DateTime<Utc>,
    pub date_modified: Option<DateTime<Utc>>,
    pub profile_id: String,
    pub is_folder: bool,
    pub children: Vec<Bookmark>, // For nested serialization
}

pub struct BookmarkManager {
    conn: Arc<RwLock<Connection>>,
    db_path: PathBuf,
}

impl BookmarkManager {
    pub async fn new() -> CoreResult<Self> {
        let db_path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Ardia")
            .join("bookmarks.db");

        std::fs::create_dir_all(db_path.parent().unwrap())?;

        let conn = Connection::open(&db_path)?;

        // Create tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS bookmarks (
                id TEXT PRIMARY KEY,
                parent_id TEXT,
                index INTEGER NOT NULL,
                title TEXT NOT NULL,
                url TEXT,
                date_added TEXT NOT NULL,
                date_modified TEXT,
                profile_id TEXT NOT NULL,
                is_folder INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_bookmarks_parent ON bookmarks(parent_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_bookmarks_profile ON bookmarks(profile_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_bookmarks_url ON bookmarks(url)",
            [],
        )?;

        // Create root folders if they don't exist
        Self::ensure_root_folders(&conn).await?;

        Ok(Self {
            conn: Arc::new(RwLock::new(conn)),
            db_path,
        })
    }

    async fn ensure_root_folders(conn: &Connection) -> CoreResult<()> {
        let roots = [
            ("bookmarks_bar", "Bookmarks Bar"),
            ("other_bookmarks", "Other Bookmarks"),
            ("mobile_bookmarks", "Mobile Bookmarks"),
        ];

        for (id_str, title) in roots {
            let id = Uuid::parse_str(id_str).unwrap_or_else(|_| Uuid::new_v4());
            let exists: bool = conn
                .query_row(
                    "SELECT 1 FROM bookmarks WHERE id = ?1",
                    params![id.to_string()],
                    |_| Ok(true),
                )
                .optional()?
                .is_some();

            if !exists {
                conn.execute(
                    "INSERT INTO bookmarks (id, parent_id, index, title, url, date_added, profile_id, is_folder)
                     VALUES (?1, NULL, ?2, ?3, NULL, ?4, 'default', 1)",
                    params![id.to_string(), 0, title, Utc::now().to_rfc3339()],
                )?;
            }
        }

        Ok(())
    }

    pub async fn add_bookmark(
        &self,
        parent_id: BookmarkId,
        index: i32,
        title: String,
        url: Option<String>,
        profile_id: String,
    ) -> CoreResult<BookmarkId> {
        let conn = self.conn.read().await;

        let id = BookmarkId::new();
        let now = Utc::now();

        conn.execute(
            "INSERT INTO bookmarks (id, parent_id, index, title, url, date_added, profile_id, is_folder)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                id.to_string(),
                parent_id.to_string(),
                index,
                &title,
                url.as_deref(),
                now.to_rfc3339(),
                &profile_id,
                0, // not a folder
            ],
        )?;

        // Update indices of siblings after the insertion point
        conn.execute(
            "UPDATE bookmarks SET index = index + 1 WHERE parent_id = ?1 AND index >= ?2 AND id != ?3",
            params![parent_id.to_string(), index, id.to_string()],
        )?;

        Ok(id)
    }

    pub async fn add_folder(
        &self,
        parent_id: BookmarkId,
        index: i32,
        title: String,
        profile_id: String,
    ) -> CoreResult<BookmarkId> {
        let conn = self.conn.read().await;

        let id = BookmarkId::new();
        let now = Utc::now();

        conn.execute(
            "INSERT INTO bookmarks (id, parent_id, index, title, url, date_added, profile_id, is_folder)
             VALUES (?1, ?2, ?3, ?4, NULL, ?5, ?6, 1)",
            params![id.to_string(), parent_id.to_string(), index, &title, now.to_rfc3339(), &profile_id],
        )?;

        conn.execute(
            "UPDATE bookmarks SET index = index + 1 WHERE parent_id = ?1 AND index >= ?2 AND id != ?3",
            params![parent_id.to_string(), index, id.to_string()],
        )?;

        Ok(id)
    }

    pub async fn get_bookmark(&self, id: BookmarkId) -> CoreResult<Option<Bookmark>> {
        let conn = self.conn.read().await;

        let bookmark = conn
            .query_row(
                "SELECT id, parent_id, index, title, url, date_added, date_modified, profile_id, is_folder
                 FROM bookmarks WHERE id = ?1",
                params![id.to_string()],
                |row| {
                    Ok(Bookmark {
                        id: BookmarkId(Uuid::parse_str(&row.get::<_, String>(0)?).unwrap()),
                        parent_id: row.get::<_, Option<String>>(1)?.map(|s| BookmarkId(Uuid::parse_str(&s).unwrap())),
                        index: row.get(2)?,
                        title: row.get(3)?,
                        url: row.get(4)?,
                        date_added: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?).unwrap().with_timezone(&Utc),
                        date_modified: row.get::<_, Option<String>>(6)?.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                        profile_id: row.get(7)?,
                        is_folder: row.get(8)?,
                        children: Vec::new(),
                    })
                },
            )
            .optional()?;

        Ok(bookmark)
    }

    pub async fn get_children(&self, parent_id: BookmarkId, profile_id: &str) -> CoreResult<Vec<Bookmark>> {
        let conn = self.conn.read().await;

        let mut stmt = conn.prepare(
            "SELECT id, parent_id, index, title, url, date_added, date_modified, profile_id, is_folder
             FROM bookmarks
             WHERE parent_id = ?1 AND profile_id = ?2
             ORDER BY index",
        )?;

        let children = stmt
            .query_map(params![parent_id.to_string(), profile_id], |row| {
                Ok(Bookmark {
                    id: BookmarkId(Uuid::parse_str(&row.get::<_, String>(0)?).unwrap()),
                    parent_id: row.get::<_, Option<String>>(1)?.map(|s| BookmarkId(Uuid::parse_str(&s).unwrap())),
                    index: row.get(2)?,
                    title: row.get(3)?,
                    url: row.get(4)?,
                    date_added: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?).unwrap().with_timezone(&Utc),
                    date_modified: row.get::<_, Option<String>>(6)?.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                    profile_id: row.get(7)?,
                    is_folder: row.get(8)?,
                    children: Vec::new(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(children)
    }

    pub async fn get_tree(&self, profile_id: &str) -> CoreResult<Vec<Bookmark>> {
        let roots = [
            BookmarkId(Uuid::parse_str("bookmarks_bar").unwrap_or_else(|_| Uuid::new_v4())),
            BookmarkId(Uuid::parse_str("other_bookmarks").unwrap_or_else(|_| Uuid::new_v4())),
            BookmarkId(Uuid::parse_str("mobile_bookmarks").unwrap_or_else(|_| Uuid::new_v4())),
        ];

        let mut result = Vec::new();
        for root_id in roots {
            if let Some(root) = self.get_bookmark(root_id).await? {
                let mut root_with_children = self.build_tree(root, profile_id).await?;
                result.push(root_with_children);
            }
        }

        Ok(result)
    }

    async fn build_tree(&self, mut bookmark: Bookmark, profile_id: &str) -> CoreResult<Bookmark> {
        if bookmark.is_folder {
            let children = self.get_children(bookmark.id, profile_id).await?;
            let mut child_tree = Vec::new();
            for child in children {
                child_tree.push(self.build_tree(child, profile_id).await?);
            }
            bookmark.children = child_tree;
        }
        Ok(bookmark)
    }

    pub async fn update_bookmark(
        &self,
        id: BookmarkId,
        title: Option<String>,
        url: Option<String>,
    ) -> CoreResult<()> {
        let conn = self.conn.read().await;
        let now = Utc::now();

        let mut updates = Vec::new();
        let mut params_vec = Vec::new();

        if let Some(title) = title {
            updates.push("title = ?");
            params_vec.push(title);
        }
        if let Some(url) = url {
            updates.push("url = ?");
            params_vec.push(url);
        }

        if !updates.is_empty() {
            updates.push("date_modified = ?");
            params_vec.push(now.to_rfc3339());
            params_vec.push(id.to_string());

            let query = format!("UPDATE bookmarks SET {} WHERE id = ?", updates.join(", "));
            let params: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
            conn.execute(&query, rusqlite::params_from_iter(params))?;
        }

        Ok(())
    }

    pub async fn move_bookmark(
        &self,
        id: BookmarkId,
        new_parent_id: BookmarkId,
        new_index: i32,
    ) -> CoreResult<()> {
        let conn = self.conn.read().await;

        // Get current parent
        let old_parent: Option<String> = conn
            .query_row(
                "SELECT parent_id FROM bookmarks WHERE id = ?1",
                params![id.to_string()],
                |row| row.get(0),
            )
            .optional()?;

        if let Some(old_parent_str) = old_parent {
            let old_parent_id = BookmarkId(Uuid::parse_str(&old_parent_str).unwrap());

            // Update indices in old parent
            conn.execute(
                "UPDATE bookmarks SET index = index - 1 WHERE parent_id = ?1 AND index > (SELECT index FROM bookmarks WHERE id = ?2)",
                params![old_parent_str, id.to_string()],
            )?;
        }

        // Update indices in new parent
        conn.execute(
            "UPDATE bookmarks SET index = index + 1 WHERE parent_id = ?1 AND index >= ?2",
            params![new_parent_id.to_string(), new_index],
        )?;

        // Move the bookmark
        conn.execute(
            "UPDATE bookmarks SET parent_id = ?1, index = ?2, date_modified = ?3 WHERE id = ?4",
            params![new_parent_id.to_string(), new_index, Utc::now().to_rfc3339(), id.to_string()],
        )?;

        Ok(())
    }

    pub async fn delete_bookmark(&self, id: BookmarkId) -> CoreResult<()> {
        let conn = self.conn.read().await;

        // If it's a folder, delete all children recursively
        let is_folder: bool = conn
            .query_row(
                "SELECT is_folder FROM bookmarks WHERE id = ?1",
                params![id.to_string()],
                |row| row.get(0),
            )?;

        if is_folder {
            self.delete_folder_recursive(&conn, id).await?;
        } else {
            // Update indices of siblings
            let parent_id: Option<String> = conn
                .query_row(
                    "SELECT parent_id FROM bookmarks WHERE id = ?1",
                    params![id.to_string()],
                    |row| row.get(0),
                )
                .optional()?;

            if let Some(parent_str) = parent_id {
                let index: i32 = conn
                    .query_row(
                        "SELECT index FROM bookmarks WHERE id = ?1",
                        params![id.to_string()],
                        |row| row.get(0),
                    )?;

                conn.execute(
                    "UPDATE bookmarks SET index = index - 1 WHERE parent_id = ?1 AND index > ?2",
                    params![parent_str, index],
                )?;
            }

            conn.execute("DELETE FROM bookmarks WHERE id = ?1", params![id.to_string()])?;
        }

        Ok(())
    }

    async fn delete_folder_recursive(&self, conn: &Connection, id: BookmarkId) -> CoreResult<()> {
        // Get all children
        let mut stmt = conn.prepare(
            "SELECT id FROM bookmarks WHERE parent_id = ?1",
        )?;

        let children: Vec<BookmarkId> = stmt
            .query_map(params![id.to_string()], |row| {
                Ok(BookmarkId(Uuid::parse_str(&row.get::<_, String>(0)?).unwrap()))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        // Recursively delete children
        for child_id in children {
            self.delete_folder_recursive(conn, child_id).await?;
        }

        // Delete self
        conn.execute("DELETE FROM bookmarks WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }

    pub async fn search_bookmarks(&self, profile_id: &str, query: &str) -> CoreResult<Vec<Bookmark>> {
        let conn = self.conn.read().await;

        let mut stmt = conn.prepare(
            "SELECT id, parent_id, index, title, url, date_added, date_modified, profile_id, is_folder
             FROM bookmarks
             WHERE profile_id = ?1 AND (title LIKE ?2 OR url LIKE ?2)
             ORDER BY title",
        )?;

        let search_pattern = format!("%{}%", query);

        let bookmarks = stmt
            .query_map(params![profile_id, search_pattern], |row| {
                Ok(Bookmark {
                    id: BookmarkId(Uuid::parse_str(&row.get::<_, String>(0)?).unwrap()),
                    parent_id: row.get::<_, Option<String>>(1)?.map(|s| BookmarkId(Uuid::parse_str(&s).unwrap())),
                    index: row.get(2)?,
                    title: row.get(3)?,
                    url: row.get(4)?,
                    date_added: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?).unwrap().with_timezone(&Utc),
                    date_modified: row.get::<_, Option<String>>(6)?.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                    profile_id: row.get(7)?,
                    is_folder: row.get(8)?,
                    children: Vec::new(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(bookmarks)
    }

    pub async fn export_bookmarks(&self, profile_id: &str) -> CoreResult<String> {
        let tree = self.get_tree(profile_id).await?;
        let json = serde_json::to_string_pretty(&tree)?;
        Ok(json)
    }

    pub async fn import_bookmarks(&self, profile_id: &str, json: &str) -> CoreResult<()> {
        let bookmarks: Vec<Bookmark> = serde_json::from_str(json)?;

        for bookmark in bookmarks {
            self.import_bookmark_recursive(profile_id, bookmark, None).await?;
        }

        Ok(())
    }

    async fn import_bookmark_recursive(
        &self,
        profile_id: &str,
        bookmark: Bookmark,
        parent_id: Option<BookmarkId>,
    ) -> CoreResult<()> {
        let parent = parent_id.unwrap_or_else(|| BookmarkId(Uuid::parse_str("other_bookmarks").unwrap_or_else(|_| Uuid::new_v4())));

        let new_id = if bookmark.is_folder {
            self.add_folder(parent, bookmark.index, bookmark.title, profile_id.to_string()).await?
        } else {
            self.add_bookmark(parent, bookmark.index, bookmark.title, bookmark.url, profile_id.to_string()).await?
        };

        for child in bookmark.children {
            self.import_bookmark_recursive(profile_id, child, Some(new_id)).await?;
        }

        Ok(())
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

impl Default for BookmarkManager {
    fn default() -> Self {
        Self {
            conn: Arc::new(RwLock::new(Connection::open_in_memory().unwrap())),
            db_path: PathBuf::from("."),
        }
    }
}