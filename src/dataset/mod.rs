//! Dataset management for WGSL code generation training

use serde::{Deserialize, Serialize};
use std::path::Path;

/// A single training example: natural language → WGSL code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WGSLExample {
    pub natural_language: String,
    pub wgsl_code: String,
}

/// Dataset for WGSL code generation
#[derive(Debug, Clone)]
pub struct WGSLDataset {
    pub examples: Vec<WGSLExample>,
}

impl WGSLDataset {
    /// Create a new empty dataset
    pub fn new() -> Self {
        WGSLDataset {
            examples: Vec::new(),
        }
    }

    /// Load dataset from JSON file
    pub fn from_json<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let examples: Vec<WGSLExample> = serde_json::from_str(&content)?;
        Ok(WGSLDataset { examples })
    }

    /// Load dataset from TOML file
    pub fn from_toml<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        use serde::Deserialize;

        #[derive(Deserialize)]
        struct DatasetFile {
            examples: Vec<WGSLExample>,
        }

        let content = std::fs::read_to_string(path)?;
        let data: DatasetFile = toml::from_str(&content)?;
        Ok(WGSLDataset { examples: data.examples })
    }

    /// Get number of examples
    pub fn len(&self) -> usize {
        self.examples.len()
    }

    /// Check if dataset is empty
    pub fn is_empty(&self) -> bool {
        self.examples.is_empty()
    }

    /// Split dataset into train/val/test
    pub fn split(&self, train_ratio: f32, val_ratio: f32) -> (Self, Self, Self) {
        let total = self.examples.len();
        let train_size = (total as f32 * train_ratio) as usize;
        let val_size = (total as f32 * val_ratio) as usize;

        let train_examples = self.examples[..train_size].to_vec();
        let val_examples = self.examples[train_size..train_size + val_size].to_vec();
        let test_examples = self.examples[train_size + val_size..].to_vec();

        (
            WGSLDataset { examples: train_examples },
            WGSLDataset { examples: val_examples },
            WGSLDataset { examples: test_examples },
        )
    }
}

impl Default for WGSLDataset {
    fn default() -> Self {
        Self::new()
    }
}
