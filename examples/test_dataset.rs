//! Test loading and inspecting the WGSL training dataset

use tiny_agent_trainer::dataset::WGSLDataset;

fn main() -> anyhow::Result<()> {
    println!("📚 WGSL Training Dataset Test\n");

    // Load dataset from TOML
    println!("📂 Loading dataset from: config/wgsl_training_data.toml");
    let dataset = WGSLDataset::from_toml("config/wgsl_training_data.toml")?;

    println!("✅ Dataset loaded successfully!");
    println!("   Total examples: {}\n", dataset.len());

    // Show statistics
    let total = dataset.len();
    let avg_nl_length: f32 = dataset.examples.iter()
        .map(|ex| ex.natural_language.len() as f32)
        .sum::<f32>() / total as f32;
    let avg_wgsl_length: f32 = dataset.examples.iter()
        .map(|ex| ex.wgsl_code.len() as f32)
        .sum::<f32>() / total as f32;

    println!("📊 Dataset Statistics:");
    println!("   Average NL description length: {:.1} chars", avg_nl_length);
    println!("   Average WGSL code length: {:.1} chars\n", avg_wgsl_length);

    // Show first 5 examples
    println!("📝 Sample Examples (first 5):");
    println!("{}", "=".repeat(60));

    for (i, example) in dataset.examples.iter().take(5).enumerate() {
        println!("\n#{} Natural Language:", i + 1);
        println!("   {}", example.natural_language);
        println!("   WGSL Code:");

        // Show first 100 chars of code
        let preview = if example.wgsl_code.len() > 100 {
            format!("{}...", &example.wgsl_code[..100])
        } else {
            example.wgsl_code.clone()
        };

        for line in preview.lines() {
            println!("   {}", line);
        }
    }

    println!("\n{}", "=".repeat(60));

    // Test dataset splitting
    println!("\n🔀 Testing dataset split...");
    let (train, val, test) = dataset.split(0.8, 0.1);

    println!("   Training set: {} examples ({:.1}%)", train.len(), (train.len() as f32 / total as f32) * 100.0);
    println!("   Validation set: {} examples ({:.1}%)", val.len(), (val.len() as f32 / total as f32) * 100.0);
    println!("   Test set: {} examples ({:.1}%)", test.len(), (test.len() as f32 / total as f32) * 100.0);

    // Verify split
    let split_total = train.len() + val.len() + test.len();
    assert_eq!(split_total, total, "Split totals don't match!");
    println!("   ✅ Split verified: {} total\n", split_total);

    // Categorize examples by type
    println!("🏷️  Example Categories:");
    let mut categories: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();

    for example in &dataset.examples {
        let category = if example.natural_language.contains("color") || example.natural_language.contains("Color") {
            "Colors"
        } else if example.natural_language.contains("chromatic") || example.natural_language.contains("Chromatic") {
            "Chromatic Operations"
        } else if example.natural_language.contains("fragment") {
            "Fragment Shaders"
        } else if example.natural_language.contains("compute") {
            "Compute Shaders"
        } else if example.natural_language.contains("vertex") {
            "Vertex Shaders"
        } else if example.natural_language.contains("matrix") || example.natural_language.contains("Matrix") {
            "Matrix Operations"
        } else if example.natural_language.contains("texture") || example.natural_language.contains("Texture") {
            "Texture Operations"
        } else if example.wgsl_code.contains("fn ") {
            "Functions"
        } else {
            "Other"
        };

        *categories.entry(category).or_insert(0) += 1;
    }

    let mut sorted_categories: Vec<_> = categories.iter().collect();
    sorted_categories.sort_by(|a, b| b.1.cmp(a.1));

    for (category, count) in sorted_categories {
        println!("   {}: {} examples ({:.1}%)",
            category, count, (*count as f32 / total as f32) * 100.0);
    }

    println!("\n✅ Dataset test completed successfully!");
    println!("\n💡 This dataset is ready for training!");
    println!("   Run: cargo run --release -- train --config config/wgsl_generation.toml");

    Ok(())
}
