//! Tiny Agent Trainer CLI
//!
//! Command-line interface for training and running WGSL code generation models

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tiny_agent_trainer::{init_logging, Config, WGSLValidator};

#[derive(Parser)]
#[command(name = "tiny-agent-trainer")]
#[command(about = "Train AI models to generate WGSL shader code from natural language", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Check system capabilities
    Check,

    /// List available configurations
    List {
        /// Configuration directory
        #[arg(short, long, default_value = "config")]
        config_dir: PathBuf,
    },

    /// Show configuration details
    Show {
        /// Configuration name
        #[arg(short, long)]
        config: String,
    },

    /// Train a model
    Train {
        /// Configuration file
        #[arg(short, long)]
        config: PathBuf,

        /// Override number of epochs
        #[arg(short, long)]
        epochs: Option<usize>,
    },

    /// Generate WGSL code from natural language
    Generate {
        /// Model checkpoint path
        #[arg(short, long)]
        model: PathBuf,

        /// Natural language prompt
        #[arg(short, long)]
        prompt: String,

        /// Output file (optional, prints to stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Validate WGSL code
    Validate {
        /// WGSL file to validate
        file: PathBuf,
    },

    /// Create a default configuration file
    Init {
        /// Output path for configuration
        #[arg(short, long, default_value = "config/wgsl_generation.toml")]
        output: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        std::env::set_var("RUST_LOG", "debug");
    }
    init_logging();

    match cli.command {
        Commands::Check => check_system(),
        Commands::List { config_dir } => list_configs(&config_dir),
        Commands::Show { config } => show_config(&config),
        Commands::Train { config, epochs } => train_model(&config, epochs),
        Commands::Generate {
            model,
            prompt,
            output,
        } => generate_wgsl(&model, &prompt, output.as_deref()),
        Commands::Validate { file } => validate_wgsl(&file),
        Commands::Init { output } => init_config(&output),
    }
}

fn check_system() -> anyhow::Result<()> {
    println!("🔍 System Check");
    println!("{}", "=".repeat(40));

    // Check WGPU availability
    println!("🖥️  GPU Support:");
    let adapters: Vec<_> = wgpu::Instance::default().enumerate_adapters(wgpu::Backends::all());
    match adapters.first() {
        Some(adapter) => {
            let info = adapter.get_info();
            println!("  ✅ GPU found: {}", info.name);
            println!("     Backend: {:?}", info.backend);
            println!("     Type: {:?}", info.device_type);
        }
        None => {
            println!("  ⚠️  No GPU detected");
        }
    }

    println!("\n📦 Dependencies:");
    println!("  ✅ wgpu: {}", env!("CARGO_PKG_VERSION"));
    println!("  ✅ naga: validation available");

    println!("\n✅ System check complete!");
    Ok(())
}

fn list_configs(config_dir: &PathBuf) -> anyhow::Result<()> {
    println!("📋 Available Configurations:");
    println!("{}", "=".repeat(40));

    if !config_dir.exists() {
        println!("⚠️  Config directory not found: {}", config_dir.display());
        println!("💡 Run 'tiny-agent-trainer init' to create a default config");
        return Ok(());
    }

    let mut found = false;
    for entry in std::fs::read_dir(config_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            println!("  📄 {}", path.file_stem().unwrap().to_string_lossy());
            found = true;
        }
    }

    if !found {
        println!("  No configuration files found");
    }

    Ok(())
}

fn show_config(config_name: &str) -> anyhow::Result<()> {
    let config_path = PathBuf::from(format!("config/{}.toml", config_name));
    let config = Config::from_file(&config_path)?;

    println!("🔧 Configuration: {}", config.task.name);
    println!("{}", "=".repeat(50));
    println!("\n📊 Model:");
    println!("  Architecture: {}", config.model.architecture);
    println!("  d_model: {}", config.model.d_model);
    println!("  Attention heads: {}", config.model.nhead);
    println!("  Layers: {}", config.model.num_layers);

    println!("\n🎯 Training:");
    println!("  Epochs: {}", config.training.num_epochs);
    println!("  Batch size: {}", config.training.batch_size);
    println!("  Learning rate: {}", config.training.learning_rate);
    println!("  Optimizer: {}", config.training.optimizer);

    println!("\n📚 Dataset:");
    println!("  Train path: {}", config.dataset.train_path.display());

    Ok(())
}

fn train_model(_config_path: &PathBuf, _epochs: Option<usize>) -> anyhow::Result<()> {
    println!("🚀 Training model...");
    println!("⚠️  Training not yet implemented - placeholder only");

    // TODO: Implement actual training
    // let config = Config::from_file(config_path)?;
    // let mut trainer = Trainer::new(config.training);
    // trainer.train(&mut model)?;

    Ok(())
}

fn generate_wgsl(
    _model_path: &PathBuf,
    prompt: &str,
    output: Option<&std::path::Path>,
) -> anyhow::Result<()> {
    println!("🎨 Generating WGSL code...");
    println!("Prompt: {}", prompt);

    // TODO: Load actual model
    // let generator = WGSLGenerator::from_checkpoint(model_path)?;

    // For now, use templates as examples
    let wgsl_code = if prompt.contains("mix") {
        tiny_agent_trainer::ChromaticTemplate::mix()
    } else if prompt.contains("filter") {
        tiny_agent_trainer::ChromaticTemplate::filter()
    } else if prompt.contains("complement") {
        tiny_agent_trainer::ChromaticTemplate::complement()
    } else if prompt.contains("saturate") {
        tiny_agent_trainer::ChromaticTemplate::saturate()
    } else {
        format!("// Generated WGSL for: {}\n// TODO: Train model to generate actual code\n", prompt)
    };

    if let Some(output_path) = output {
        std::fs::write(output_path, &wgsl_code)?;
        println!("✅ Saved to: {}", output_path.display());
    } else {
        println!("\n{}", wgsl_code);
    }

    Ok(())
}

fn validate_wgsl(file: &PathBuf) -> anyhow::Result<()> {
    println!("🔍 Validating WGSL: {}", file.display());

    let validator = WGSLValidator::new();
    let result = validator.validate_file(file)?;

    result.print();

    if !result.is_valid {
        std::process::exit(1);
    }

    Ok(())
}

fn init_config(output: &PathBuf) -> anyhow::Result<()> {
    println!("📝 Creating default configuration...");

    let config = Config::default_wgsl_generation();

    // Ensure parent directory exists
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    config.to_file(output)?;

    println!("✅ Configuration created: {}", output.display());
    println!("\n💡 Next steps:");
    println!("  1. Edit the configuration file as needed");
    println!("  2. Prepare training data at: config/wgsl_training_data.toml");
    println!("  3. Run: tiny-agent-trainer train --config {}", output.display());

    Ok(())
}
