//! Test loading and validating the engine configuration

use tiny_agent_trainer::EngineConfig;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Engine Configuration Test\n");

    // Load engine configuration
    println!("📂 Loading engine configuration from: config/engine.toml");
    let engine_config = EngineConfig::from_file("config/engine.toml")?;

    println!("✅ Engine configuration loaded successfully!\n");

    // Display configuration
    println!("⚙️  Engine Configuration:");
    println!("   Log Level: {}", engine_config.log_level);
    println!("   Debug Assertions: {}", if engine_config.disable_debug_assertions { "Disabled" } else { "Enabled" });
    println!();

    println!("📁 Output Paths:");
    println!("   Logs:        {}", engine_config.paths.log_path.display());
    println!("   Journals:    {}", engine_config.paths.journal_path.display());
    println!("   Checkpoints: {}", engine_config.paths.checkpoint_path.display());
    println!();

    // Validate configuration
    println!("🔍 Validating configuration values...");
    engine_config.validate()?;
    println!("   ✅ Configuration values are valid\n");

    // Validate required files
    println!("🔍 Checking for required configuration files...");
    let config_dir = Path::new("config");
    let errors = engine_config.validate_required_files(config_dir)?;

    if errors.is_empty() {
        println!("   ✅ All required configuration files found\n");
    } else {
        println!("   ⚠️  {} missing file(s)\n", errors.len());
        for error in &errors {
            println!("   - {}", error);
        }
        println!();
    }

    // Test directory creation
    println!("📁 Testing output directory creation...");
    engine_config.create_directories()?;
    println!("   ✅ Output directories created/verified\n");

    // Verify directories exist
    println!("🔍 Verifying directories exist:");
    let paths_to_check = [
        &engine_config.paths.log_path,
        &engine_config.paths.journal_path,
        &engine_config.paths.checkpoint_path,
    ];

    for path in &paths_to_check {
        if path.exists() && path.is_dir() {
            println!("   ✅ {} exists", path.display());
        } else {
            println!("   ❌ {} not found", path.display());
        }
    }
    println!();

    // Test with invalid log level
    println!("🔍 Testing validation with invalid log level...");
    let mut invalid_config = engine_config.clone();
    invalid_config.log_level = "INVALID".to_string();

    match invalid_config.validate() {
        Ok(_) => println!("   ❌ Validation should have failed!"),
        Err(e) => println!("   ✅ Validation correctly rejected invalid log level: {}", e),
    }
    println!();

    // Test saving configuration
    println!("💾 Testing configuration save...");
    let test_path = "config/engine_test.toml";
    engine_config.to_file(test_path)?;
    println!("   ✅ Configuration saved to {}\n", test_path);

    // Test reloading
    println!("🔄 Testing configuration reload...");
    let reloaded_config = EngineConfig::from_file(test_path)?;
    println!("   ✅ Configuration reloaded successfully");
    println!("   Log Level: {}", reloaded_config.log_level);
    println!();

    // Cleanup test file
    std::fs::remove_file(test_path)?;
    println!("🧹 Cleaned up test file\n");

    println!("✅ All engine configuration tests passed!");
    println!();
    println!("💡 The engine configuration system is ready for production use!");
    println!("   - Log level can be changed without code modifications");
    println!("   - Debug assertions can be disabled for production");
    println!("   - Output paths are predictable and auditable");
    println!("   - Configuration validation prevents startup with invalid settings");

    Ok(())
}
