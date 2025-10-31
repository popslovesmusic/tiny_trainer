# Quick Reference Guide

Fast reference for common tasks with Tiny Agent Trainer.

## Installation

```bash
cargo build --release
./target/release/tiny-agent-trainer --version
```

## Essential Commands

| Command | Purpose | Example |
|---------|---------|---------|
| `check` | Verify system | `tiny-agent-trainer check` |
| `init` | Create config | `tiny-agent-trainer init` |
| `generate` | Generate WGSL | `tiny-agent-trainer generate --model dummy --prompt "mix colors"` |
| `validate` | Check WGSL | `tiny-agent-trainer validate shader.wgsl` |
| `list` | List configs | `tiny-agent-trainer list` |
| `show` | Show config | `tiny-agent-trainer show --config wgsl_generation` |

## Common Workflows

### Generate and Validate

```bash
tiny-agent-trainer generate --model dummy --prompt "TEXT" --output file.wgsl
tiny-agent-trainer validate file.wgsl
```

### Quick Template Generation

```bash
# Mix operation
tiny-agent-trainer generate --model dummy --prompt "mix"

# Filter operation
tiny-agent-trainer generate --model dummy --prompt "filter"

# Complement
tiny-agent-trainer generate --model dummy --prompt "complement"

# Saturate
tiny-agent-trainer generate --model dummy --prompt "saturate"
```

## Template Trigger Words

| Keyword | Generated Template |
|---------|-------------------|
| `mix` | Chromatic mix (additive coherence) |
| `filter` | Chromatic filter (subtractive) |
| `complement` | Chromatic complement (hue rotation) |
| `saturate` | Chromatic saturate (saturation adjust) |

## Configuration Quick Edit

```toml
# config/wgsl_generation.toml

# Model size (larger = better quality, slower)
d_model = 512        # 256, 512, 768, 1024
nhead = 8            # 4, 8, 12, 16
num_layers = 6       # 2, 4, 6, 8, 12

# Training speed
batch_size = 16      # 8, 16, 32, 64
learning_rate = 0.0001  # 0.001, 0.0001, 0.00001
```

## Validation Error Fixes

| Error | Fix |
|-------|-----|
| Missing @location | Add `@location(N)` to parameters/return |
| Wrong vec size | Check vec2/vec3/vec4 matches usage |
| Type mismatch | Ensure consistent types (f32, u32, i32) |
| Invalid binding | Add `@group(N) @binding(M)` |
| Syntax error | Check WGSL syntax rules |

## WGSL Snippets

### Colors
```wgsl
vec4<f32>(1.0, 0.0, 0.0, 1.0)  // Red
vec4<f32>(0.0, 1.0, 0.0, 1.0)  // Green
vec4<f32>(0.0, 0.0, 1.0, 1.0)  // Blue
vec4<f32>(1.0, 1.0, 1.0, 1.0)  // White
```

### Fragment Shader
```wgsl
@fragment
fn main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
```

### Compute Shader
```wgsl
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    // Work here
}
```

### Buffer Access
```wgsl
@group(0) @binding(0) var<storage, read> input: array<f32>;
@group(0) @binding(1) var<storage, read_write> output: array<f32>;
```

## Troubleshooting

| Problem | Solution |
|---------|----------|
| GPU not found | Update drivers, use `WGPU_BACKEND=gl` |
| Build fails | `cargo clean && cargo build` |
| Config error | `tiny-agent-trainer init` |
| Validation fails | Check WGSL syntax, use validator error messages |

## Environment Variables

```bash
export RUST_LOG=debug                # Enable debug logging
export WGPU_BACKEND=vulkan          # Force Vulkan backend
export WGPU_POWER_PREF=high         # Use discrete GPU
```

## File Locations

```
tiny_agent_trainer_rs/
├── config/
│   ├── wgsl_generation.toml         # Main config
│   └── wgsl_training_data.toml      # Training data (150+ examples)
├── target/release/
│   └── tiny-agent-trainer.exe       # Built binary
├── docs/
│   ├── USER_GUIDE.md                # Full guide
│   └── QUICK_REFERENCE.md           # This file
└── README.md                        # Project overview
```

## Training Data Format

```toml
[[examples]]
natural_language = "Description in English"
wgsl_code = "corresponding WGSL code"

[[examples]]
natural_language = "Another example"
wgsl_code = """Multi-line
WGSL code
here"""
```

## Useful Links

- [WGSL Spec](https://www.w3.org/TR/WGSL/)
- [WebGPU Samples](https://webgpu.github.io/webgpu-samples/)
- [Chromatic Cognition Core](https://github.com/yourusername/chromatic_cognition_core)
- [naga Validator](https://github.com/gfx-rs/naga)

## Performance Tips

1. **Use release build**: `cargo build --release` (10x faster)
2. **GPU acceleration**: Ensure GPU drivers updated
3. **Batch operations**: Process multiple shaders together
4. **Cache results**: Save validated shaders
5. **Optimize config**: Lower batch_size if memory limited

## Common Patterns

### Generate Multiple Shaders
```bash
for op in mix filter complement saturate; do
    tiny-agent-trainer generate --model dummy \\
        --prompt "$op operation" \\
        --output "${op}.wgsl"
done
```

### Validate Directory
```bash
for file in shaders/*.wgsl; do
    echo "Validating $file..."
    tiny-agent-trainer validate "$file"
done
```

### Auto-generate Config
```bash
tiny-agent-trainer init --output config/my_custom.toml
```

---

**For detailed information, see [USER_GUIDE.md](USER_GUIDE.md)**
