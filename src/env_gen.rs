use anyhow::{Context, Result};
use std::fs;

use crate::storage::{Storage, SkateEntry};
use crate::colors::ColoredOutput;

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
    pub fn generate_env_file(storage: &Storage, output_path: &str, filter: Option<&str>) -> Result<()> {
        let entries = storage.list(None).context("Failed to list storage entries")?;

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

        println!("{} Generated {} environment variables to {}",
            ColoredOutput::success("Success:"),
            ColoredOutput::count(filtered_entries.len()),
            ColoredOutput::path(output_path)
        );
        Ok(())
    }

    /// Generates an environment file from entries in a specific database.
    /// 
    /// # Arguments
    /// 
    /// * `db_name` - The name of the database to generate from
    /// * `output_path` - The path where the .env file will be written
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or an error if database operations or file writing fails.
    pub fn generate_from_db(storage: &Storage, db_name: &str, output_path: &str) -> Result<()> {
        let entries = storage.list(Some(db_name)).context("Failed to list database entries")?;

        let env_content = Self::entries_to_env_format(&entries);

        fs::write(output_path, env_content)
            .with_context(|| format!("Failed to write env file to {}", output_path))?;

        println!("{} Generated {} environment variables from database {} to {}",
            ColoredOutput::success("Success:"),
            ColoredOutput::count(entries.len()),
            ColoredOutput::database(db_name),
            ColoredOutput::path(output_path)
        );
        Ok(())
    }

    /// Converts entries to environment file format.
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
    pub fn show_preview(storage: &Storage, filter: Option<&str>) -> Result<()> {
        let entries = storage.list(None).context("Failed to list storage entries")?;

        let filtered_entries = if let Some(prefix) = filter {
            entries
                .into_iter()
                .filter(|entry| entry.key.starts_with(prefix))
                .collect()
        } else {
            entries
        };

        if filtered_entries.is_empty() {
            println!("{}", ColoredOutput::warning("No entries found"));
            return Ok(());
        }

        println!("{}", ColoredOutput::header("Preview of environment variables:"));
        println!();
        for entry in &filtered_entries {
            let key = entry.key.to_uppercase().replace('-', "_").replace(' ', "_");
            println!("{}", ColoredOutput::format_env_line(&key, &Self::quote_value(&entry.value)));
        }
        println!();
        println!("{} {} variables total",
            ColoredOutput::info("Info:"),
            ColoredOutput::count(filtered_entries.len())
        );
        Ok(())
    }

    /// Exports shell variables for evaluation in shell (e.g., eval $(skatos export)).
    ///
    /// Outputs export statements that can be directly evaluated by bash/zsh.
    /// Values are properly escaped and quoted for shell safety.
    ///
    /// # Arguments
    ///
    /// * `storage` - The storage instance
    /// * `database` - Optional database name (defaults to "default")
    /// * `filter` - Optional prefix to filter entries by key name
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if reading entries fails.
    pub fn export_shell(storage: &Storage, database: Option<&str>, filter: Option<&str>) -> Result<()> {
        let entries = storage.list(database).context("Failed to list storage entries")?;

        let filtered_entries: Vec<_> = if let Some(prefix) = filter {
            entries
                .into_iter()
                .filter(|entry| entry.key.starts_with(prefix))
                .collect()
        } else {
            entries
        };

        for entry in filtered_entries {
            let key = entry.key.to_uppercase().replace('-', "_").replace(' ', "_");
            let escaped_value = Self::shell_escape(&entry.value);
            println!("export {}={}", key, escaped_value);
        }

        Ok(())
    }

    /// Escapes a value for safe shell evaluation.
    ///
    /// Uses single quotes for safety and escapes any single quotes in the value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to escape
    ///
    /// # Returns
    ///
    /// Returns the properly escaped value suitable for shell export.
    fn shell_escape(value: &str) -> String {
        format!("'{}'", value.replace('\'', "'\\''"))
    }

    /// Creates a JSON backup of all entries.
    /// 
    /// # Arguments
    /// 
    /// * `output_path` - The path where the backup file will be written
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or an error if reading entries or writing file fails.
    pub fn backup_to_file(storage: &Storage, output_path: &str) -> Result<()> {
        let entries = storage.list(None).context("Failed to list storage entries")?;
        let json = serde_json::to_string_pretty(&entries)
            .context("Failed to serialize entries to JSON")?;

        fs::write(output_path, json)
            .with_context(|| format!("Failed to write backup file to {}", output_path))?;

        println!("{} Backed up {} entries to {}",
            ColoredOutput::success("Success:"),
            ColoredOutput::count(entries.len()),
            ColoredOutput::path(output_path)
        );
        Ok(())
    }

    /// Restores entries from a JSON backup file.
    /// 
    /// # Arguments
    /// 
    /// * `input_path` - The path to the backup file to restore from
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or an error if reading file or setting entries fails.
    pub fn restore_from_file(storage: &Storage, input_path: &str) -> Result<()> {
        let content = fs::read_to_string(input_path)
            .with_context(|| format!("Failed to read backup file from {}", input_path))?;

        let entries: Vec<SkateEntry> = serde_json::from_str(&content)
            .context("Failed to parse backup file as JSON")?;

        let mut restored = 0;
        for entry in entries {
            if storage.set(&entry.key, &entry.value, None).is_ok() {
                restored += 1;
            }
        }

        println!("{} Restored {} entries from {}",
            ColoredOutput::success("Success:"),
            ColoredOutput::count(restored),
            ColoredOutput::path(input_path)
        );
        Ok(())
    }
}