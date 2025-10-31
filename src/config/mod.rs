//! Configuration management for training and inference
//!
//! This module provides TOML-based configuration following the chromatic_cognition_core pattern.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Task name and metadata
    pub task: TaskConfig,
    /// Model architecture configuration
    pub model: ModelConfig,
    /// Training parameters
    pub training: TrainingConfig,
    /// Tokenizer settings
    pub tokenizer: TokenizerConfig,
    /// Dataset configuration
    pub dataset: DatasetConfig,
}

/// Task-level configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    /// Task name (e.g., "wgsl_generation")
    pub name: String,
    /// Task type (e.g., "code_generation")
    pub task_type: String,
    /// Description
    #[serde(default)]
    pub description: String,
}

/// Model architecture configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Architecture type ("transformer", "lstm", etc.)
    pub architecture: String,
    /// Embedding dimension
    pub d_model: usize,
    /// Number of attention heads
    pub nhead: usize,
    /// Number of encoder/decoder layers
    pub num_layers: usize,
    /// Feedforward dimension
    #[serde(default = "default_dim_feedforward")]
    pub dim_feedforward: usize,
    /// Dropout rate
    #[serde(default = "default_dropout")]
    pub dropout: f32,
    /// Maximum sequence length
    #[serde(default = "default_max_seq_len")]
    pub max_seq_len: usize,
}

/// Training configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// Number of epochs
    pub num_epochs: usize,
    /// Batch size
    pub batch_size: usize,
    /// Learning rate
    pub learning_rate: f64,
    /// Optimizer ("adam", "adamw", "sgd")
    #[serde(default = "default_optimizer")]
    pub optimizer: String,
    /// Enable early stopping
    #[serde(default = "default_true")]
    pub early_stopping: bool,
    /// Early stopping patience
    #[serde(default = "default_patience")]
    pub early_stopping_patience: usize,
    /// Gradient clipping norm
    #[serde(default = "default_grad_clip")]
    pub gradient_clip_norm: f64,
    /// Save checkpoint every N epochs
    #[serde(default = "default_save_every")]
    pub save_every: usize,
}

/// Tokenizer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizerConfig {
    /// Tokenizer type ("wgsl", "word", "char")
    #[serde(default = "default_tokenizer_type")]
    pub tokenizer_type: String,
    /// Maximum token length
    #[serde(default = "default_max_length")]
    pub max_length: usize,
    /// Convert to lowercase
    #[serde(default)]
    pub lowercase: bool,
    /// Minimum frequency for vocabulary
    #[serde(default = "default_min_freq")]
    pub min_freq: usize,
}

/// Dataset configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetConfig {
    /// Path to training data file
    pub train_path: PathBuf,
    /// Optional validation data path
    pub val_path: Option<PathBuf>,
    /// Optional test data path
    pub test_path: Option<PathBuf>,
    /// Training split ratio (if no separate val/test)
    #[serde(default = "default_train_ratio")]
    pub train_ratio: f32,
    /// Validation split ratio
    #[serde(default = "default_val_ratio")]
    pub val_ratio: f32,
}

/// Engine configuration for production environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Global logging level (DEBUG, INFO, WARN, ERROR)
    #[serde(default = "default_log_level")]
    pub log_level: String,
    /// Disable debug assertions in production
    #[serde(default)]
    pub disable_debug_assertions: bool,
    /// Output paths configuration
    pub paths: PathsConfig,
}

/// Output path configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsConfig {
    /// Log file output directory
    #[serde(default = "default_log_path")]
    pub log_path: PathBuf,
    /// Journal file output directory
    #[serde(default = "default_journal_path")]
    pub journal_path: PathBuf,
    /// Checkpoint file output directory
    #[serde(default = "default_checkpoint_path")]
    pub checkpoint_path: PathBuf,
}

// Default value functions
fn default_dim_feedforward() -> usize {
    2048
}

fn default_dropout() -> f32 {
    0.1
}

fn default_max_seq_len() -> usize {
    512
}

fn default_optimizer() -> String {
    "adamw".to_string()
}

fn default_true() -> bool {
    true
}

fn default_patience() -> usize {
    10
}

fn default_grad_clip() -> f64 {
    1.0
}

fn default_save_every() -> usize {
    10
}

fn default_tokenizer_type() -> String {
    "wgsl".to_string()
}

fn default_max_length() -> usize {
    512
}

fn default_min_freq() -> usize {
    1
}

fn default_train_ratio() -> f32 {
    0.8
}

fn default_val_ratio() -> f32 {
    0.1
}

fn default_log_level() -> String {
    "INFO".to_string()
}

fn default_log_path() -> PathBuf {
    PathBuf::from("logs/")
}

fn default_journal_path() -> PathBuf {
    PathBuf::from("journals/")
}

fn default_checkpoint_path() -> PathBuf {
    PathBuf::from("checkpoints/")
}

impl Config {
    /// Load configuration from TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to TOML file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> crate::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Create a default configuration for WGSL generation
    pub fn default_wgsl_generation() -> Self {
        Config {
            task: TaskConfig {
                name: "wgsl_generation".to_string(),
                task_type: "code_generation".to_string(),
                description: "Generate WGSL shader code from natural language".to_string(),
            },
            model: ModelConfig {
                architecture: "transformer".to_string(),
                d_model: 512,
                nhead: 8,
                num_layers: 6,
                dim_feedforward: 2048,
                dropout: 0.1,
                max_seq_len: 512,
            },
            training: TrainingConfig {
                num_epochs: 100,
                batch_size: 16,
                learning_rate: 0.0001,
                optimizer: "adamw".to_string(),
                early_stopping: true,
                early_stopping_patience: 15,
                gradient_clip_norm: 1.0,
                save_every: 10,
            },
            tokenizer: TokenizerConfig {
                tokenizer_type: "wgsl".to_string(),
                max_length: 512,
                lowercase: false,
                min_freq: 1,
            },
            dataset: DatasetConfig {
                train_path: PathBuf::from("config/wgsl_training_data.toml"),
                val_path: None,
                test_path: None,
                train_ratio: 0.8,
                val_ratio: 0.1,
            },
        }
    }
}

impl EngineConfig {
    /// Load engine configuration from TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let path_ref = path.as_ref();
        if !path_ref.exists() {
            return Err(crate::Error::ConfigError(format!(
                "Engine configuration file not found: {}",
                path_ref.display()
            )));
        }

        let content = std::fs::read_to_string(path_ref)?;
        let config: EngineConfig = toml::from_str(&content)?;

        // Validate log level
        config.validate()?;

        Ok(config)
    }

    /// Save engine configuration to TOML file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> crate::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Create a default engine configuration
    pub fn default() -> Self {
        EngineConfig {
            log_level: "INFO".to_string(),
            disable_debug_assertions: false,
            paths: PathsConfig::default(),
        }
    }

    /// Validate configuration values
    pub fn validate(&self) -> crate::Result<()> {
        // Validate log level
        let valid_levels = ["DEBUG", "INFO", "WARN", "ERROR"];
        if !valid_levels.contains(&self.log_level.as_str()) {
            return Err(crate::Error::ConfigError(format!(
                "Invalid log_level '{}'. Must be one of: DEBUG, INFO, WARN, ERROR",
                self.log_level
            )));
        }

        // Ensure paths are not empty
        if self.paths.log_path.as_os_str().is_empty() {
            return Err(crate::Error::ConfigError(
                "log_path cannot be empty".to_string()
            ));
        }
        if self.paths.journal_path.as_os_str().is_empty() {
            return Err(crate::Error::ConfigError(
                "journal_path cannot be empty".to_string()
            ));
        }
        if self.paths.checkpoint_path.as_os_str().is_empty() {
            return Err(crate::Error::ConfigError(
                "checkpoint_path cannot be empty".to_string()
            ));
        }

        Ok(())
    }

    /// Validate required configuration files exist
    pub fn validate_required_files(&self, config_dir: &Path) -> crate::Result<Vec<String>> {
        let mut errors = Vec::new();

        // List of required configuration files
        let required_files = [
            "wgsl_generation.toml",
            "wgsl_training_data.toml",
        ];

        for file in &required_files {
            let file_path = config_dir.join(file);
            if !file_path.exists() {
                errors.push(format!(
                    "Required configuration file missing: {}",
                    file_path.display()
                ));
            }
        }

        if !errors.is_empty() {
            eprintln!("❌ Configuration validation errors:");
            for error in &errors {
                eprintln!("   - {}", error);
            }
        }

        Ok(errors)
    }

    /// Create all output directories
    pub fn create_directories(&self) -> crate::Result<()> {
        std::fs::create_dir_all(&self.paths.log_path)?;
        std::fs::create_dir_all(&self.paths.journal_path)?;
        std::fs::create_dir_all(&self.paths.checkpoint_path)?;
        Ok(())
    }
}

impl PathsConfig {
    /// Create a default paths configuration
    pub fn default() -> Self {
        PathsConfig {
            log_path: PathBuf::from("logs/"),
            journal_path: PathBuf::from("journals/"),
            checkpoint_path: PathBuf::from("checkpoints/"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default_wgsl_generation();
        assert_eq!(config.task.name, "wgsl_generation");
        assert_eq!(config.model.architecture, "transformer");
        assert_eq!(config.model.d_model, 512);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default_wgsl_generation();
        let toml_str = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.task.name, deserialized.task.name);
    }
}
