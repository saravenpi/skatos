use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkateEntry {
    pub key: String,
    pub value: String,
}

pub struct Skate;

impl Skate {
    /// Sets a key-value pair in the skate store.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key to set
    /// * `value` - The value to associate with the key
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or an error if the skate command fails.
    pub fn set(key: &str, value: &str) -> Result<()> {
        let output = Command::new("skate")
            .args(["set", key, value])
            .output()
            .context("Failed to execute skate set command")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Skate set failed: {}", error);
        }

        Ok(())
    }

    /// Retrieves the value for a given key from the skate store.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key to look up
    /// 
    /// # Returns
    /// 
    /// Returns the value associated with the key, or an error if not found or command fails.
    pub fn get(key: &str) -> Result<String> {
        let output = Command::new("skate")
            .args(["get", key])
            .output()
            .context("Failed to execute skate get command")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Skate get failed: {}", error);
        }

        let value = String::from_utf8(output.stdout)
            .context("Failed to parse skate output as UTF-8")?
            .trim()
            .to_string();

        Ok(value)
    }

    /// Lists all key-value pairs from the skate store.
    /// 
    /// # Returns
    /// 
    /// Returns a vector of `SkateEntry` structs containing all stored key-value pairs,
    /// or an error if the skate command fails.
    pub fn list() -> Result<Vec<SkateEntry>> {
        let output = Command::new("skate")
            .args(["list"])
            .output()
            .context("Failed to execute skate list command")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Skate list failed: {}", error);
        }

        let output_str = String::from_utf8(output.stdout)
            .context("Failed to parse skate output as UTF-8")?;

        let mut entries = Vec::new();
        for line in output_str.lines() {
            if let Some((key, value)) = line.split_once('\t') {
                entries.push(SkateEntry {
                    key: key.to_string(),
                    value: value.to_string(),
                });
            }
        }

        Ok(entries)
    }

    /// Lists all keys from the skate store without their values.
    /// 
    /// # Returns
    /// 
    /// Returns a vector of strings containing all stored keys,
    /// or an error if the skate command fails.
    pub fn list_keys() -> Result<Vec<String>> {
        let output = Command::new("skate")
            .args(["list", "--keys-only"])
            .output()
            .context("Failed to execute skate list --keys-only command")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Skate list --keys-only failed: {}", error);
        }

        let output_str = String::from_utf8(output.stdout)
            .context("Failed to parse skate output as UTF-8")?;

        Ok(output_str.lines().map(|s| s.to_string()).collect())
    }

    /// Lists all available databases in the skate store.
    /// 
    /// # Returns
    /// 
    /// Returns a vector of database names,
    /// or an error if the skate command fails.
    pub fn list_dbs() -> Result<Vec<String>> {
        let output = Command::new("skate")
            .args(["list-dbs"])
            .output()
            .context("Failed to execute skate list-dbs command")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Skate list-dbs failed: {}", error);
        }

        let output_str = String::from_utf8(output.stdout)
            .context("Failed to parse skate output as UTF-8")?;

        Ok(output_str.lines().map(|s| s.to_string()).collect())
    }

    /// Deletes a key-value pair from the skate store.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key to delete
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or an error if the key doesn't exist or command fails.
    pub fn delete(key: &str) -> Result<()> {
        let output = Command::new("skate")
            .args(["delete", key])
            .output()
            .context("Failed to execute skate delete command")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Skate delete failed: {}", error);
        }

        Ok(())
    }

    /// Converts a slice of SkateEntry structs into a HashMap for easier manipulation.
    /// 
    /// # Arguments
    /// 
    /// * `entries` - Slice of SkateEntry structs to convert
    /// 
    /// # Returns
    /// 
    /// Returns a HashMap mapping keys to values.
    pub fn to_env_map(entries: &[SkateEntry]) -> HashMap<String, String> {
        entries
            .iter()
            .map(|entry| (entry.key.clone(), entry.value.clone()))
            .collect()
    }
}