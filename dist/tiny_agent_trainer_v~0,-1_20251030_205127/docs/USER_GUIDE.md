# Tiny Agent Trainer - User Guide

Complete guide to using the Tiny Agent Trainer for WGSL code generation.

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [CLI Commands](#cli-commands)
4. [Configuration](#configuration)
5. [Training](#training)
6. [Code Generation](#code-generation)
7. [Validation](#validation)
8. [Examples](#examples)
9. [Troubleshooting](#troubleshooting)
10. [Advanced Usage](#advanced-usage)

---

## Installation

### Prerequisites

- **Rust 1.70 or later**: Install from [rustup.rs](https://rustup.rs/)
- **GPU Drivers** (optional): For GPU-accelerated features
  - NVIDIA: Latest GeForce/Quadro drivers
  - AMD: Latest Radeon drivers
  - Intel: Latest integrated graphics drivers

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/tiny_agent_trainer_rs.git
cd tiny_agent_trainer_rs

# Build in release mode (optimized)
cargo build --release

# The binary will be at: ./target/release/tiny-agent-trainer
```

### Verify Installation

```bash
./target/release/tiny-agent-trainer --version
./target/release/tiny-agent-trainer check
```

---

## Quick Start

### 1. System Check

First, verify your system capabilities:

```bash
./target/release/tiny-agent-trainer check
```

**Expected Output:**
```
🔍 System Check
========================================
🖥️  GPU Support:
  ✅ GPU found: NVIDIA GeForce RTX 3080
     Backend: Vulkan
     Type: DiscreteGpu

📦 Dependencies:
  ✅ wgpu: 2.0.0
  ✅ naga: validation available

✅ System check complete!
```

### 2. Initialize Configuration

Create a default configuration file:

```bash
./target/release/tiny-agent-trainer init
```

This creates `config/wgsl_generation.toml` with default settings.

### 3. Generate Your First Shader

Generate a simple WGSL shader:

```bash
./target/release/tiny-agent-trainer generate \\
    --model dummy \\
    --prompt "create chromatic mix operation" \\
    --output my_first_shader.wgsl
```

### 4. Validate the Shader

Verify the generated WGSL is valid:

```bash
./target/release/tiny-agent-trainer validate my_first_shader.wgsl
```

**Success Output:**
```
🔍 Validating WGSL: my_first_shader.wgsl
✅ WGSL code is valid
```

---

## CLI Commands

### `check` - System Validation

Check system capabilities and GPU availability.

```bash
tiny-agent-trainer check [--verbose]
```

**Options:**
- `--verbose`: Show detailed logging information

**Use Cases:**
- Verify installation
- Check GPU availability
- Troubleshoot system issues

---

### `init` - Initialize Configuration

Create a default configuration file.

```bash
tiny-agent-trainer init [--output PATH]
```

**Options:**
- `--output PATH`: Custom output path (default: `config/wgsl_generation.toml`)

**Example:**
```bash
tiny-agent-trainer init --output my_config.toml
```

---

### `list` - List Configurations

List all available configuration files.

```bash
tiny-agent-trainer list [--config-dir DIR]
```

**Options:**
- `--config-dir DIR`: Configuration directory (default: `config`)

**Example:**
```bash
tiny-agent-trainer list --config-dir ./my_configs
```

---

### `show` - Show Configuration Details

Display detailed information about a configuration.

```bash
tiny-agent-trainer show --config NAME
```

**Example:**
```bash
tiny-agent-trainer show --config wgsl_generation
```

**Output:**
```
🔧 Configuration: wgsl_generation
==================================================

📊 Model:
  Architecture: transformer
  d_model: 512
  Attention heads: 8
  Layers: 6

🎯 Training:
  Epochs: 100
  Batch size: 16
  Learning rate: 0.0001
  Optimizer: adamw

📚 Dataset:
  Train path: config/wgsl_training_data.toml
```

---

### `generate` - Generate WGSL Code

Generate WGSL shader code from natural language prompts.

```bash
tiny-agent-trainer generate \\
    --model PATH \\
    --prompt "DESCRIPTION" \\
    [--output FILE]
```

**Options:**
- `--model PATH`: Path to trained model (use `dummy` for templates)
- `--prompt TEXT`: Natural language description
- `--output FILE`: Save to file (optional, prints to stdout if omitted)

**Examples:**

Generate and print to console:
```bash
tiny-agent-trainer generate \\
    --model dummy \\
    --prompt "create chromatic mix operation"
```

Generate and save to file:
```bash
tiny-agent-trainer generate \\
    --model dummy \\
    --prompt "create chromatic filter" \\
    --output filter.wgsl
```

**Available Templates (with `--model dummy`):**
- Prompts containing "mix" → Chromatic mix operation
- Prompts containing "filter" → Chromatic filter operation
- Prompts containing "complement" → Chromatic complement
- Prompts containing "saturate" → Chromatic saturate

---

### `validate` - Validate WGSL Code

Validate WGSL shader code for syntax and semantic errors.

```bash
tiny-agent-trainer validate FILE
```

**Example:**
```bash
tiny-agent-trainer validate shader.wgsl
```

**Success Output:**
```
🔍 Validating WGSL: shader.wgsl
✅ WGSL code is valid
```

**Error Output:**
```
🔍 Validating WGSL: bad_shader.wgsl
❌ WGSL validation failed:
  - Parse error: unexpected token 'bad' at line 3
```

---

### `train` - Train a Model

Train a new model on WGSL generation task (requires training data).

```bash
tiny-agent-trainer train \\
    --config CONFIG_FILE \\
    [--epochs N]
```

**Options:**
- `--config FILE`: Configuration file path
- `--epochs N`: Override number of epochs

**Example:**
```bash
tiny-agent-trainer train \\
    --config config/wgsl_generation.toml \\
    --epochs 50
```

**Note:** Training requires a complete training dataset and transformer model implementation (in progress).

---

## Configuration

Configuration files use TOML format and control all aspects of training and generation.

### Configuration Structure

```toml
[task]
name = "wgsl_generation"
task_type = "code_generation"
description = "Generate WGSL shader code from natural language"

[model]
architecture = "transformer"      # Model type
d_model = 512                     # Embedding dimension
nhead = 8                         # Number of attention heads
num_layers = 6                    # Transformer layers
dim_feedforward = 2048            # FFN dimension
dropout = 0.1                     # Dropout rate
max_seq_len = 512                 # Maximum sequence length

[training]
num_epochs = 100                  # Training epochs
batch_size = 16                   # Batch size
learning_rate = 0.0001            # Learning rate
optimizer = "adamw"               # Optimizer (adam, adamw, sgd)
early_stopping = true             # Enable early stopping
early_stopping_patience = 15      # Patience for early stopping
gradient_clip_norm = 1.0          # Gradient clipping
save_every = 10                   # Save checkpoint every N epochs

[tokenizer]
tokenizer_type = "wgsl"           # Tokenizer type
max_length = 512                  # Max token length
lowercase = false                 # Convert to lowercase
min_freq = 1                      # Minimum token frequency

[dataset]
train_path = "config/wgsl_training_data.toml"
val_path = ""                     # Optional validation set
test_path = ""                    # Optional test set
train_ratio = 0.8                 # Training split ratio
val_ratio = 0.1                   # Validation split ratio
```

### Editing Configuration

1. **Create default**: `tiny-agent-trainer init`
2. **Edit file**: Open `config/wgsl_generation.toml` in your editor
3. **Modify values**: Adjust parameters as needed
4. **Validate**: `tiny-agent-trainer show --config wgsl_generation`

### Common Adjustments

**For faster training:**
```toml
[training]
num_epochs = 50
batch_size = 32
learning_rate = 0.001
```

**For better quality:**
```toml
[model]
d_model = 768
nhead = 12
num_layers = 8
```

**For limited memory:**
```toml
[training]
batch_size = 8
[model]
d_model = 256
num_layers = 4
```

---

## Training

### Training Data Format

Training data uses TOML format with natural language → WGSL pairs:

```toml
[[examples]]
natural_language = "Create a red color"
wgsl_code = "vec4<f32>(1.0, 0.0, 0.0, 1.0)"

[[examples]]
natural_language = "Normalize a vector"
wgsl_code = "normalize(vec3<f32>(x, y, z))"

[[examples]]
natural_language = "Simple fragment shader"
wgsl_code = """@fragment
fn main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}"""
```

### Creating Training Data

1. **Use provided dataset**: `config/wgsl_training_data.toml` (150+ examples)
2. **Add custom examples**: Edit the file to include your own patterns
3. **Validate format**: Ensure TOML syntax is correct

### Training Workflow

```bash
# 1. Prepare configuration
tiny-agent-trainer init

# 2. Verify dataset
cat config/wgsl_training_data.toml

# 3. Start training
tiny-agent-trainer train --config config/wgsl_generation.toml

# 4. Monitor progress
# (Training logs will show epoch, loss, accuracy)

# 5. Use trained model
tiny-agent-trainer generate \\
    --model models/best_model.bin \\
    --prompt "create a blue color"
```

---

## Code Generation

### Using Templates

Generate code using built-in templates (no training required):

```bash
# Chromatic mix
tiny-agent-trainer generate --model dummy \\
    --prompt "create chromatic mix operation"

# Chromatic filter
tiny-agent-trainer generate --model dummy \\
    --prompt "apply chromatic filter"

# Chromatic complement
tiny-agent-trainer generate --model dummy \\
    --prompt "compute chromatic complement"

# Chromatic saturate
tiny-agent-trainer generate --model dummy \\
    --prompt "saturate color"
```

### Using Trained Models

Once you have a trained model:

```bash
tiny-agent-trainer generate \\
    --model models/my_model.bin \\
    --prompt "create a fragment shader with UV gradient" \\
    --output gradient.wgsl
```

### Best Practices

1. **Be specific**: "fragment shader with red color" vs "red"
2. **Use domain terms**: "compute shader", "workgroup size", "binding"
3. **Mention operations**: "mix colors", "normalize vector", "clamp value"
4. **Reference structures**: "with vec4", "using mat4x4", "array of f32"

### Example Prompts

**Good prompts:**
- "Create a compute shader that adds two arrays of vec4"
- "Fragment shader that samples a texture at UV coordinates"
- "Normalize a 3D vector"
- "Chromatic mix operation for two tensors"
- "Calculate diffuse lighting with surface normal"

**Poor prompts:**
- "shader" (too vague)
- "make it work" (no specifics)
- "the thing" (unclear)

---

## Validation

### What Validation Checks

The validator uses `naga` (official WGSL parser) to check:

1. **Syntax**: Proper WGSL grammar
2. **Semantics**: Type correctness, valid operations
3. **Entry points**: Valid shader stages (@compute, @fragment, @vertex)
4. **Bindings**: Correct @group and @binding annotations

### Validation Examples

**Valid WGSL:**
```wgsl
@fragment
fn main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(uv.x, uv.y, 0.0, 1.0);
}
```

**Invalid WGSL:**
```wgsl
@fragment
fn main() -> vec4<f32> {  // Missing @location(0)
    return vec4<f32>(1.0);  // Wrong vec4 constructor
}
```

### Validation Workflow

```bash
# 1. Generate code
tiny-agent-trainer generate --model dummy \\
    --prompt "fragment shader" \\
    --output test.wgsl

# 2. Validate
tiny-agent-trainer validate test.wgsl

# 3. Fix errors if needed
nano test.wgsl

# 4. Re-validate
tiny-agent-trainer validate test.wgsl
```

---

## Examples

### Example 1: Simple Color Shader

```bash
# Generate
tiny-agent-trainer generate --model dummy \\
    --prompt "create a red color" \\
    --output red.wgsl

# Content of red.wgsl (template fallback):
# // Generated WGSL for: create a red color
# // TODO: Train model to generate actual code
```

### Example 2: Chromatic Mix

```bash
# Generate chromatic mix operation
tiny-agent-trainer generate --model dummy \\
    --prompt "mix two colors" \\
    --output mix.wgsl

# Validate
tiny-agent-trainer validate mix.wgsl
# Output: ✅ WGSL code is valid

# View result
cat mix.wgsl
```

### Example 3: Batch Processing

```bash
# Generate multiple shaders
for op in "mix" "filter" "complement" "saturate"; do
    tiny-agent-trainer generate --model dummy \\
        --prompt "chromatic $op operation" \\
        --output "${op}.wgsl"

    tiny-agent-trainer validate "${op}.wgsl"
done
```

### Example 4: Custom Workflow

```bash
#!/bin/bash
# generate_and_validate.sh

PROMPT="$1"
OUTPUT="$2"

echo "Generating shader..."
./target/release/tiny-agent-trainer generate \\
    --model dummy \\
    --prompt "$PROMPT" \\
    --output "$OUTPUT"

echo "Validating shader..."
./target/release/tiny-agent-trainer validate "$OUTPUT"

if [ $? -eq 0 ]; then
    echo "✅ Success! Shader saved to $OUTPUT"
else
    echo "❌ Validation failed"
    exit 1
fi
```

Usage:
```bash
chmod +x generate_and_validate.sh
./generate_and_validate.sh "create mix operation" mix.wgsl
```

---

## Troubleshooting

### GPU Not Detected

**Problem:** `⚠️ No GPU detected` when running `check`

**Solutions:**
1. Update GPU drivers
2. Check GPU is enabled in BIOS
3. Try different backend: Set `WGPU_BACKEND=dx12` (Windows) or `WGPU_BACKEND=vulkan`
4. CPU mode still works, just slower

### Validation Fails

**Problem:** Generated shader fails validation

**Solutions:**
1. Check for syntax errors in prompt
2. Use templates first (`--model dummy`)
3. Manually fix and re-validate
4. Report issue if template fails

### Configuration Errors

**Problem:** `Config file not found` or `Invalid TOML`

**Solutions:**
1. Run `tiny-agent-trainer init` to create default
2. Check TOML syntax: [TOML Validator](https://www.toml-lint.com/)
3. Verify file path is correct
4. Check file permissions

### Build Errors

**Problem:** Compilation fails

**Solutions:**
1. Update Rust: `rustup update`
2. Clear build cache: `cargo clean`
3. Check dependencies: `cargo check`
4. Install build tools (Windows: Visual Studio Build Tools)

### Out of Memory

**Problem:** Training fails with OOM error

**Solutions:**
1. Reduce batch size in config
2. Reduce model size (d_model, num_layers)
3. Close other applications
4. Use gradient checkpointing (future feature)

---

## Advanced Usage

### Environment Variables

```bash
# Logging level
export RUST_LOG=debug
export RUST_LOG=tiny_agent_trainer=trace

# WGPU backend selection
export WGPU_BACKEND=vulkan  # vulkan, dx12, metal, gl

# Disable GPU
export WGPU_BACKEND=gl
```

### Programmatic Usage

Use as a Rust library:

```rust
use tiny_agent_trainer::{Config, WGSLValidator, ChromaticTemplate};

fn main() -> anyhow::Result<()> {
    // Load config
    let config = Config::from_file("config/wgsl_generation.toml")?;

    // Validate WGSL
    let validator = WGSLValidator::new();
    let wgsl = ChromaticTemplate::mix();
    let result = validator.validate(&wgsl)?;

    assert!(result.is_valid);
    Ok(())
}
```

### Custom Templates

Add your own templates in code:

```rust
impl ChromaticTemplate {
    pub fn my_custom_operation() -> String {
        r#"
        @compute @workgroup_size(64)
        fn my_operation(@builtin(global_invocation_id) id: vec3<u32>) {
            // Your custom WGSL here
        }
        "#.to_string()
    }
}
```

### Integration with Chromatic Cognition Core

Use generated shaders with chromatic_cognition_core:

```rust
use tiny_agent_trainer::ChromaticTemplate;
use chromatic_cognition_core::tensor::ChromaticTensor;

// Generate shader
let wgsl = ChromaticTemplate::mix();

// Use with wgpu in chromatic system
let device = /* wgpu device */;
let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("chromatic_mix"),
    source: wgpu::ShaderSource::Wgsl(wgsl.into()),
});
```

---

## Next Steps

1. **Learn WGSL**: [WebGPU Shading Language Spec](https://www.w3.org/TR/WGSL/)
2. **Explore Examples**: Check `examples/` directory
3. **Read Source**: Browse `src/` for implementation details
4. **Contribute**: Add more training examples or improve templates
5. **Integrate**: Use with chromatic_cognition_core project

---

## Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/tiny_agent_trainer_rs/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/tiny_agent_trainer_rs/discussions)
- **Documentation**: [Full API Docs](https://docs.rs/tiny_agent_trainer)

---

**Happy Shader Generating! 🎨**
