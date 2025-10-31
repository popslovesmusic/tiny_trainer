//! Tiny Agent Trainer - WGSL Code Generation from Natural Language
//!
//! A Rust framework for training AI models to generate WGSL (WebGPU Shading Language)
//! shader code from natural language descriptions. Inspired by chromatic_cognition_core.
//!
//! # Features
//!
//! - **WGSL Tokenization**: Specialized tokenizer for WGSL syntax
//! - **Transformer Models**: Encoder-decoder architecture for code generation
//! - **GPU Acceleration**: Native wgpu support for training and inference
//! - **WGSL Validation**: Integrated naga validation for generated shaders
//! - **Chromatic Operations**: Pre-built templates for color-based tensor operations
//!
//! # Example
//!
//! ```no_run
//! use tiny_agent_trainer::{Config, WGSLValidator};
//!
//! // Load configuration
//! let config = Config::from_file("config/wgsl_generation.toml")?;
//!
//! // Validate WGSL code
//! let validator = WGSLValidator::new();
//! let wgsl_code = r#"
//!     @fragment
//!     fn main() -> @location(0) vec4<f32> {
//!         return vec4<f32>(1.0, 0.0, 0.0, 1.0);
//!     }
//! "#;
//! let result = validator.validate(wgsl_code)?;
//! assert!(result.is_valid);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod config;
pub mod dataset;
pub mod inference;
pub mod model;
pub mod tokenizer;
pub mod training;
pub mod wgsl;

// Re-export commonly used types
pub use config::{Config, DatasetConfig, EngineConfig, ModelConfig, PathsConfig, TokenizerConfig, TrainingConfig};
pub use inference::WGSLGenerator;
pub use tokenizer::WGSLTokenizer;
pub use training::Trainer;
pub use wgsl::{ChromaticTemplate, WGSLValidator};

/// Custom error types for the library
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerError(#[from] toml::ser::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("{0}")]
    Other(String),
}

/// Result type alias using custom Error
pub type Result<T> = std::result::Result<T, Error>;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize logging with sensible defaults
pub fn init_logging() {
    use tracing_subscriber::{fmt, EnvFilter};

    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
}

/// Initialize logging from engine configuration
pub fn init_logging_from_config(engine_config: &EngineConfig) {
    use tracing_subscriber::{fmt, EnvFilter};

    let level = engine_config.log_level.to_lowercase();
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&level)),
        )
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
