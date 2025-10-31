//! Basic usage example for Tiny Agent Trainer library
//!
//! This example demonstrates:
//! - Loading configuration
//! - Creating a tokenizer
//! - Validating WGSL code
//! - Using chromatic templates

use tiny_agent_trainer::{
    ChromaticTemplate, Config, WGSLTokenizer, WGSLValidator,
};

fn main() -> anyhow::Result<()> {
    println!("🎨 Tiny Agent Trainer - Basic Usage Example\n");

    // 1. Initialize logging
    tiny_agent_trainer::init_logging();

    // 2. Create default configuration
    println!("📝 Creating default configuration...");
    let config = Config::default_wgsl_generation();
    println!("✅ Config created: {}", config.task.name);
    println!("   Model: {} with {} layers\n", config.model.architecture, config.model.num_layers);

    // 3. Create and test tokenizer
    println!("🔤 Testing WGSL tokenizer...");
    let mut tokenizer = WGSLTokenizer::new(512, false);

    let test_code = r#"
        @fragment
        fn main() -> @location(0) vec4<f32> {
            return vec4<f32>(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let tokens = tokenizer.tokenize(test_code);
    println!("✅ Tokenized {} tokens from test code", tokens.len());
    println!("   Sample tokens: {:?}\n", &tokens[..tokens.len().min(10)]);

    // Fit tokenizer on some sample texts
    let training_texts = vec![
        "fn main() { }",
        "var x: f32 = 1.0;",
        "@compute @workgroup_size(8, 8, 1)",
    ];
    tokenizer.fit(&training_texts, 1);
    println!("✅ Tokenizer vocabulary size: {}\n", tokenizer.vocab_size());

    // 4. Test chromatic templates
    println!("🎨 Testing chromatic templates...");

    let templates = vec![
        ("Mix", ChromaticTemplate::mix()),
        ("Filter", ChromaticTemplate::filter()),
        ("Complement", ChromaticTemplate::complement()),
        ("Saturate", ChromaticTemplate::saturate()),
    ];

    // 5. Validate each template
    println!("🔍 Validating templates...");
    let validator = WGSLValidator::new();

    for (name, wgsl_code) in templates {
        print!("   {}: ", name);
        match validator.validate(&wgsl_code) {
            Ok(result) => {
                if result.is_valid {
                    println!("✅ Valid");
                } else {
                    println!("❌ Invalid");
                    for error in result.errors {
                        println!("      Error: {}", error);
                    }
                }
            }
            Err(e) => {
                println!("❌ Validation error: {}", e);
            }
        }
    }

    println!("\n✅ Basic usage example completed successfully!");
    println!("\n💡 Next steps:");
    println!("   - Run the CLI: cargo run --release -- check");
    println!("   - Generate shaders: cargo run --release -- generate --model dummy --prompt \"mix colors\"");
    println!("   - See USER_GUIDE.md for more examples");

    Ok(())
}
