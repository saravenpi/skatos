use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkateEntry {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    entries: HashMap<String, String>,
}

impl Database {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

pub struct Storage {
    base_path: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let home_dir = dirs::home_dir().context("Could not find home directory")?;
        let base_path = home_dir.join(".skatos");

        if !base_path.exists() {
            fs::create_dir_all(&base_path)
                .context("Failed to create .skatos directory")?;
        }

        Ok(Self { base_path })
    }

    fn db_path(&self, database: &str) -> PathBuf {
        self.base_path.join(format!("{}.yml", database))
    }

    fn load_db(&self, database: &str) -> Result<Database> {
        let path = self.db_path(database);

        if !path.exists() {
            return Ok(Database::new());
        }

        let content = fs::read_to_string(&path)
            .context(format!("Failed to read database: {}", database))?;

        let db: Database = serde_yaml::from_str(&content)
            .context(format!("Failed to parse database: {}", database))?;

        Ok(db)
    }

    fn save_db(&self, database: &str, db: &Database) -> Result<()> {
        let path = self.db_path(database);
        let content = serde_yaml::to_string(&db)
            .context("Failed to serialize database")?;

        fs::write(&path, content)
            .context(format!("Failed to write database: {}", database))?;

        Ok(())
    }

    pub fn set(&self, key: &str, value: &str, database: Option<&str>) -> Result<()> {
        let db_name = database.unwrap_or("default");
        let mut db = self.load_db(db_name)?;

        db.entries.insert(key.to_string(), value.to_string());

        self.save_db(db_name, &db)?;
        Ok(())
    }

    pub fn get(&self, key: &str, database: Option<&str>) -> Result<Option<String>> {
        let db_name = database.unwrap_or("default");
        let db = self.load_db(db_name)?;

        Ok(db.entries.get(key).cloned())
    }

    pub fn delete(&self, key: &str, database: Option<&str>) -> Result<bool> {
        let db_name = database.unwrap_or("default");
        let mut db = self.load_db(db_name)?;

        let existed = db.entries.remove(key).is_some();

        if existed {
            self.save_db(db_name, &db)?;
        }

        Ok(existed)
    }

    pub fn list(&self, database: Option<&str>) -> Result<Vec<SkateEntry>> {
        let db_name = database.unwrap_or("default");
        let db = self.load_db(db_name)?;

        let mut entries: Vec<SkateEntry> = db
            .entries
            .iter()
            .map(|(k, v)| SkateEntry {
                key: k.clone(),
                value: v.clone(),
            })
            .collect();

        entries.sort_by(|a, b| a.key.cmp(&b.key));

        Ok(entries)
    }

    pub fn list_keys(&self, database: Option<&str>) -> Result<Vec<String>> {
        let db_name = database.unwrap_or("default");
        let db = self.load_db(db_name)?;

        let mut keys: Vec<String> = db.entries.keys().cloned().collect();
        keys.sort();

        Ok(keys)
    }

    pub fn list_databases(&self) -> Result<Vec<String>> {
        let entries = fs::read_dir(&self.base_path)
            .context("Failed to read .skatos directory")?;

        let mut databases = Vec::new();

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yml" {
                        if let Some(name) = path.file_stem() {
                            databases.push(name.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        databases.sort();
        Ok(databases)
    }

    #[allow(dead_code)]
    pub fn delete_database(&self, database: &str) -> Result<()> {
        let path = self.db_path(database);

        if path.exists() {
            fs::remove_file(&path)
                .context(format!("Failed to delete database: {}", database))?;
        }

        Ok(())
    }

    pub fn import_from_skate(&self) -> Result<usize> {
        use std::process::Command;

        let output = Command::new("skate")
            .arg("list-dbs")
            .output()
            .context("Failed to execute skate list-dbs")?;

        if !output.status.success() {
            anyhow::bail!("skate list-dbs failed");
        }

        let dbs_output = String::from_utf8_lossy(&output.stdout);
        let databases: Vec<&str> = dbs_output
            .lines()
            .filter(|line| line.starts_with('@'))
            .map(|line| line.trim_start_matches('@'))
            .collect();

        let mut total_imported = 0;

        for db_name in databases {
            let list_output = Command::new("skate")
                .arg("list")
                .arg(format!("@{}", db_name))
                .output()
                .context(format!("Failed to list entries for database: {}", db_name))?;

            if !list_output.status.success() {
                continue;
            }

            let entries_output = String::from_utf8_lossy(&list_output.stdout);
            let mut db = Database::new();

            for line in entries_output.lines() {
                if let Some((key, value)) = line.split_once('\t') {
                    db.entries.insert(key.to_string(), value.to_string());
                    total_imported += 1;
                }
            }

            if !db.entries.is_empty() {
                self.save_db(db_name, &db)?;
            }
        }

        Ok(total_imported)
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new().expect("Failed to initialize storage")
    }
}
