use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::skate::{Skate, SkateEntry};

pub struct EnvGenerator;

impl EnvGenerator {
    /// Generates an environment file from skate entries with optional filtering.
    /// 
    /// # Arguments
    /// 
    /// * `output_path` - The path where the .env file will be written
    /// * `filter` - Optional prefix to filter entries by key name
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or an error if reading entries or writing file fails.
    pub fn generate_env_file(output_path: &str, filter: Option<&str>) -> Result<()> {
        let entries = Skate::list().context("Failed to list skate entries")?;
        
        let filtered_entries = if let Some(prefix) = filter {
            entries
                .into_iter()
                .filter(|entry| entry.key.starts_with(prefix))
                .collect()
        } else {
            entries
        };

        let env_content = Self::entries_to_env_format(&filtered_entries);
        
        fs::write(output_path, env_content)
            .with_context(|| format!("Failed to write env file to {}", output_path))?;

        println!("Generated {} environment variables to {}", filtered_entries.len(), output_path);
        Ok(())
    }

    /// Generates an environment file from entries in a specific skate database.
    /// 
    /// # Arguments
    /// 
    /// * `db_name` - The name of the database to generate from
    /// * `output_path` - The path where the .env file will be written
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or an error if database operations or file writing fails.
    pub fn generate_from_db(db_name: &str, output_path: &str) -> Result<()> {
        let keys = Skate::list_keys().context("Failed to list skate keys")?;
        
        let db_keys: Vec<String> = keys
            .into_iter()
            .filter(|key| key.contains(&format!("@{}", db_name)))
            .collect();

        let mut entries = Vec::new();
        for key in db_keys {
            if let Ok(value) = Skate::get(&key) {
                let clean_key = key.replace(&format!("@{}", db_name), "");
                entries.push(SkateEntry {
                    key: clean_key,
                    value,
                });
            }
        }

        let env_content = Self::entries_to_env_format(&entries);
        
        fs::write(output_path, env_content)
            .with_context(|| format!("Failed to write env file to {}", output_path))?;

        println!("Generated {} environment variables from database '{}' to {}", entries.len(), db_name, output_path);
        Ok(())
    }

    /// Converts skate entries to environment file format.
    /// 
    /// Keys are converted to uppercase with dashes and spaces replaced by underscores.
    /// Values containing spaces, newlines, or quotes are automatically quoted.
    /// 
    /// # Arguments
    /// 
    /// * `entries` - Slice of SkateEntry structs to convert
    /// 
    /// # Returns
    /// 
    /// Returns a formatted string ready for writing to an .env file.
    pub fn entries_to_env_format(entries: &[SkateEntry]) -> String {
        entries
            .iter()
            .map(|entry| {
                let key = entry.key.to_uppercase().replace('-', "_").replace(' ', "_");
                format!("{}={}", key, Self::quote_value(&entry.value))
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Quotes a value if it contains special characters.
    /// 
    /// Values containing spaces, newlines, or quotes are wrapped in quotes
    /// and internal quotes are escaped.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to potentially quote
    /// 
    /// # Returns
    /// 
    /// Returns the value, quoted if necessary.
    fn quote_value(value: &str) -> String {
        if value.contains(' ') || value.contains('\n') || value.contains('"') {
            format!("\"{}\"", value.replace('"', "\\\""))
        } else {
            value.to_string()
        }
    }

    /// Shows a preview of environment variables without writing to file.
    /// 
    /// # Arguments
    /// 
    /// * `filter` - Optional prefix to filter entries by key name
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or an error if reading entries fails.
    pub fn show_preview(filter: Option<&str>) -> Result<()> {
        let entries = Skate::list().context("Failed to list skate entries")?;
        
        let filtered_entries = if let Some(prefix) = filter {
            entries
                .into_iter()
                .filter(|entry| entry.key.starts_with(prefix))
                .collect()
        } else {
            entries
        };

        if filtered_entries.is_empty() {
            println!("No entries found");
            return Ok(());
        }

        println!("Preview of environment variables:");
        println!("{}", Self::entries_to_env_format(&filtered_entries));
        Ok(())
    }

    /// Creates a JSON backup of all skate entries.
    /// 
    /// # Arguments
    /// 
    /// * `output_path` - The path where the backup file will be written
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or an error if reading entries or writing file fails.
    pub fn backup_to_file(output_path: &str) -> Result<()> {
        let entries = Skate::list().context("Failed to list skate entries")?;
        let json = serde_json::to_string_pretty(&entries)
            .context("Failed to serialize entries to JSON")?;
        
        fs::write(output_path, json)
            .with_context(|| format!("Failed to write backup file to {}", output_path))?;

        println!("Backed up {} entries to {}", entries.len(), output_path);
        Ok(())
    }

    /// Restores skate entries from a JSON backup file.
    /// 
    /// # Arguments
    /// 
    /// * `input_path` - The path to the backup file to restore from
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or an error if reading file or setting entries fails.
    pub fn restore_from_file(input_path: &str) -> Result<()> {
        let content = fs::read_to_string(input_path)
            .with_context(|| format!("Failed to read backup file from {}", input_path))?;
        
        let entries: Vec<SkateEntry> = serde_json::from_str(&content)
            .context("Failed to parse backup file as JSON")?;

        let mut restored = 0;
        for entry in entries {
            if Skate::set(&entry.key, &entry.value).is_ok() {
                restored += 1;
            }
        }

        println!("Restored {} entries from {}", restored, input_path);
        Ok(())
    }
}