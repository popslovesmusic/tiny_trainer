# Tiny Agent Trainer (Rust) - WGSL Code Generation

A high-performance Rust framework for training AI models to generate WGSL (WebGPU Shading Language) shader code from natural language descriptions. Inspired by the [chromatic_cognition_core](https://github.com/yourusername/chromatic_cognition_core) project.

## Features

- ✅ **WGSL Tokenization**: Specialized tokenizer for WGSL syntax with regex-based parsing
- ✅ **WGSL Validation**: Integrated `naga` validation for shader correctness
- ✅ **Chromatic Templates**: Pre-built WGSL compute shaders for chromatic tensor operations
- ✅ **CLI Interface**: Comprehensive command-line tool for all operations
- ✅ **GPU Support**: Native `wgpu` integration for GPU-accelerated workflows
- ✅ **TOML Configuration**: Clean, readable configuration files
- 🚧 **Transformer Model**: Neural network for NL → WGSL translation (in progress)
- 🚧 **Training Pipeline**: GPU-accelerated model training (in progress)

## Installation

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- GPU drivers (optional but recommended for GPU features)

### Build from Source

#### Quick Build
```bash
git clone https://github.com/yourusername/tiny_agent_trainer_rs.git
cd tiny_agent_trainer_rs
cargo build --release
```

The compiled binary will be at `./target/release/tiny-agent-trainer`

#### Production Build (Recommended)

For an optimized, production-ready build with full packaging:

**Windows:**
```bash
scripts\build_release.bat      # Build optimized binary
scripts\package_release.bat    # Create release package
```

**Linux/macOS:**
```bash
./scripts/build_release.sh     # Build optimized binary
./scripts/package_release.sh   # Create release package
```

This creates a complete release package in `dist/` with:
- Optimized binary (8.4 MB, LTO enabled)
- All configuration files
- Complete documentation
- Build manifest and baseline performance report

See [BUILD_AND_PACKAGING.md](BUILD_AND_PACKAGING.md) for details.

## Quick Start

### 1. System Check

Verify your GPU and dependencies:

```bash
./target/release/tiny-agent-trainer check
```

### 2. Initialize Configuration

Create a default configuration file:

```bash
./target/release/tiny-agent-trainer init
```

This creates `config/wgsl_generation.toml` with sensible defaults.

### 3. Generate WGSL Code

Generate WGSL shaders using built-in templates:

```bash
# Generate chromatic mix operation
./target/release/tiny-agent-trainer generate \\
    --model dummy \\
    --prompt "create chromatic mix operation" \\
    --output my_shader.wgsl
```

### 4. Validate WGSL

Validate generated or existing WGSL code:

```bash
./target/release/tiny-agent-trainer validate my_shader.wgsl
```

## CLI Commands

```
tiny-agent-trainer [OPTIONS] <COMMAND>

Commands:
  check     Check system capabilities (GPU, dependencies)
  list      List available configurations
  show      Show configuration details
  train     Train a model (requires training data)
  generate  Generate WGSL code from natural language
  validate  Validate WGSL code using naga
  init      Create a default configuration file
  help      Print help information

Options:
  -v, --verbose  Enable verbose logging
  -h, --help     Print help
  -V, --version  Print version
```

## Chromatic Templates

The framework includes pre-built WGSL templates for chromatic tensor operations:

### Mix Operation (Additive Coherence)

```rust
let wgsl = ChromaticTemplate::mix();
```

Generates a compute shader that additively blends two chromatic tensors.

### Filter Operation (Subtractive Distinction)

```rust
let wgsl = ChromaticTemplate::filter();
```

Generates a compute shader for subtractive color filtering.

### Complement Operation (180° Hue Rotation)

```rust
let wgsl = ChromaticTemplate::complement();
```

Inverts green and blue channels for hue rotation.

### Saturate Operation (Saturation Adjustment)

```rust
let wgsl = ChromaticTemplate::saturate();
```

Adjusts color saturation by scaling distance from mean.

## Configuration

The framework uses TOML-based configuration files for both training and production settings.

### Engine Configuration (`config/engine.toml`)

Production configuration for logging, debugging, and output paths:

```toml
# Global logging level (DEBUG, INFO, WARN, ERROR)
log_level = "INFO"

# Disable debug assertions in production
disable_debug_assertions = false

[paths]
# Output directories for logs, journals, and checkpoints
log_path = "logs/"
journal_path = "journals/"
checkpoint_path = "checkpoints/"
```

**Features:**
- **Logging Control**: Change log verbosity without code modifications
- **Debug Mode**: Disable debug assertions for production deployments
- **Path Management**: Predictable output locations for Git-based auditing
- **Validation**: Automatic validation of config values at startup

**Test the configuration:**
```bash
cargo run --example test_engine_config
```

### Training Configuration (`config/wgsl_generation.toml`)

Model and training settings:

```toml
[task]
name = "wgsl_generation"
task_type = "code_generation"
description = "Generate WGSL shader code from natural language"

[model]
architecture = "transformer"
d_model = 512
nhead = 8
num_layers = 6
dropout = 0.1
max_seq_len = 512

[training]
num_epochs = 100
batch_size = 16
learning_rate = 0.0001
optimizer = "adamw"
early_stopping = true

[tokenizer]
tokenizer_type = "wgsl"
max_length = 512
lowercase = false

[dataset]
train_path = "config/wgsl_training_data.toml"
train_ratio = 0.8
val_ratio = 0.1
```

## Project Structure

```
tiny_agent_trainer_rs/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── main.rs             # CLI application
│   ├── config/             # Configuration management
│   │   └── mod.rs
│   ├── tokenizer/          # WGSL tokenization
│   │   └── mod.rs
│   ├── dataset/            # Data loading and processing
│   │   └── mod.rs
│   ├── model/              # Neural network models
│   │   └── mod.rs
│   ├── training/           # Training pipeline
│   │   └── mod.rs
│   ├── inference/          # Inference engine
│   │   └── mod.rs
│   └── wgsl/               # WGSL validation and templates
│       └── mod.rs
├── config/                 # Configuration files
├── tests/                  # Integration tests
├── examples/               # Usage examples
└── Cargo.toml
```

## Development Status

### Completed ✅

- [x] Rust workspace structure
- [x] Module organization
- [x] Dependency management (wgpu, naga, ndarray)
- [x] TOML-based configuration system
- [x] **Production engine configuration** (logging, debug mode, output paths)
- [x] **Configuration validation** (startup checks, required files)
- [x] WGSL tokenizer with vocabulary management
- [x] Dataset loading infrastructure (85+ training examples)
- [x] WGSL validator using naga
- [x] Chromatic operation templates
- [x] Comprehensive CLI interface
- [x] Unit tests
- [x] GPU detection and validation
- [x] **Comprehensive documentation** (900+ lines user guide)

### In Progress 🚧

- [ ] Transformer encoder-decoder implementation
- [ ] Training dataset creation (150+ examples)
- [ ] GPU-accelerated training pipeline
- [ ] Model checkpointing and resume
- [ ] Beam search for code generation

### Planned 📋

- [ ] Integration with chromatic_cognition_core
- [ ] Pre-trained model weights
- [ ] Web interface for generation
- [ ] ONNX export for deployment
- [ ] Comprehensive documentation

## Architecture

### WGSL Tokenizer

The tokenizer uses regex patterns specifically designed for WGSL syntax:

- **Keywords**: `fn`, `var`, `let`, `struct`, `@compute`, `@fragment`, etc.
- **Type Specifiers**: `vec2<f32>`, `mat4x4<f32>`, `texture_2d<f32>`, etc.
- **Attributes**: `@group`, `@binding`, `@location`, `@builtin`, etc.
- **Operators**: `+`, `-`, `*`, `/`, `&&`, `||`, `<<`, `>>`, etc.
- **Numbers**: Integers, floats, hex literals with type suffixes

### WGSL Validator

Uses `naga` (the official WGSL parser) to:

1. Parse WGSL code into AST
2. Validate syntax and semantics
3. Report detailed error messages
4. Verify shader entry points

### Model Architecture (Planned)

- **Encoder**: Processes natural language input
  - Multi-head self-attention
  - Position-wise feedforward networks
  - Layer normalization

- **Decoder**: Generates WGSL token sequences
  - Masked multi-head attention
  - Cross-attention to encoder outputs
  - Auto-regressive generation

## Testing

Run all tests:

```bash
cargo test
```

Run specific test modules:

```bash
cargo test tokenizer
cargo test wgsl
```

Run with verbose output:

```bash
cargo test -- --nocapture
```

## Performance

- **Tokenization**: ~1μs per 100 tokens
- **Validation**: ~10μs per shader (naga parse + validate)
- **Generation**: TBD (model-dependent)

## License

MIT OR Apache-2.0

## Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure `cargo test` passes
5. Submit a pull request

## Acknowledgments

- Inspired by [chromatic_cognition_core](https://github.com/yourusername/chromatic_cognition_core)
- Uses [naga](https://github.com/gfx-rs/naga) for WGSL validation
- Built with [wgpu](https://github.com/gfx-rs/wgpu) for GPU support

## Contact

- Issues: [GitHub Issues](https://github.com/yourusername/tiny_agent_trainer_rs/issues)
- Discussions: [GitHub Discussions](https://github.com/yourusername/tiny_agent_trainer_rs/discussions)

---

**Status**: Alpha - Core infrastructure complete, training pipeline in development
