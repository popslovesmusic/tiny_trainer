//! WGSL validation and template generation using naga

use naga::front::wgsl;
use std::path::Path;

/// WGSL validator using naga
pub struct WGSLValidator {
    /// Whether to show warnings
    pub show_warnings: bool,
}

impl WGSLValidator {
    /// Create a new validator
    pub fn new() -> Self {
        Self {
            show_warnings: true,
        }
    }

    /// Validate WGSL code
    pub fn validate(&self, code: &str) -> crate::Result<ValidationResult> {
        match wgsl::parse_str(code) {
            Ok(module) => {
                // Perform validation
                match naga::valid::Validator::new(
                    naga::valid::ValidationFlags::all(),
                    naga::valid::Capabilities::all(),
                )
                .validate(&module)
                {
                    Ok(_) => Ok(ValidationResult {
                        is_valid: true,
                        errors: Vec::new(),
                        warnings: Vec::new(),
                    }),
                    Err(e) => Ok(ValidationResult {
                        is_valid: false,
                        errors: vec![format!("Validation error: {:?}", e)],
                        warnings: Vec::new(),
                    }),
                }
            }
            Err(e) => Ok(ValidationResult {
                is_valid: false,
                errors: vec![format!("Parse error: {}", e)],
                warnings: Vec::new(),
            }),
        }
    }

    /// Validate WGSL code from file
    pub fn validate_file<P: AsRef<Path>>(&self, path: P) -> crate::Result<ValidationResult> {
        let code = std::fs::read_to_string(path)?;
        self.validate(&code)
    }
}

impl Default for WGSLValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    /// Print validation results
    pub fn print(&self) {
        if self.is_valid {
            println!("✅ WGSL code is valid");
        } else {
            println!("❌ WGSL validation failed:");
            for error in &self.errors {
                println!("  - {}", error);
            }
        }

        if !self.warnings.is_empty() {
            println!("⚠️  Warnings:");
            for warning in &self.warnings {
                println!("  - {}", warning);
            }
        }
    }
}

/// Pre-built WGSL templates for chromatic operations
pub struct ChromaticTemplate;

impl ChromaticTemplate {
    /// Generate chromatic mix operation
    pub fn mix() -> String {
        r#"// Chromatic mix operation - additive coherence
@group(0) @binding(0) var<storage, read> tensor_a: array<vec4<f32>>;
@group(0) @binding(1) var<storage, read> tensor_b: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read_write> output: array<vec4<f32>>;

@compute @workgroup_size(8, 8, 1)
fn chromatic_mix(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x + id.y * 8u;
    let a = tensor_a[idx];
    let b = tensor_b[idx];

    // Additive blend and normalize
    let mixed = normalize(a.rgb + b.rgb);
    let certainty = (a.w + b.w) * 0.5;

    output[idx] = vec4<f32>(mixed, certainty);
}
"#
        .to_string()
    }

    /// Generate chromatic filter operation
    pub fn filter() -> String {
        r#"// Chromatic filter operation - subtractive distinction
@group(0) @binding(0) var<storage, read> tensor_a: array<vec4<f32>>;
@group(0) @binding(1) var<storage, read> tensor_b: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read_write> output: array<vec4<f32>>;

@compute @workgroup_size(8, 8, 1)
fn chromatic_filter(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x + id.y * 8u;
    let a = tensor_a[idx];
    let b = tensor_b[idx];

    // Subtractive blend, clamped to [0, 1]
    let filtered = clamp(a.rgb - b.rgb, vec3<f32>(0.0), vec3<f32>(1.0));

    output[idx] = vec4<f32>(filtered, a.w);
}
"#
        .to_string()
    }

    /// Generate chromatic complement operation
    pub fn complement() -> String {
        r#"// Chromatic complement operation - 180° hue rotation
@group(0) @binding(0) var<storage, read> tensor: array<vec4<f32>>;
@group(0) @binding(1) var<storage, read_write> output: array<vec4<f32>>;

@compute @workgroup_size(8, 8, 1)
fn chromatic_complement(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x + id.y * 8u;
    let color = tensor[idx];

    // Invert green and blue channels (hue rotation)
    let complement = vec3<f32>(color.r, 1.0 - color.g, 1.0 - color.b);

    output[idx] = vec4<f32>(complement, color.w);
}
"#
        .to_string()
    }

    /// Generate chromatic saturate operation
    pub fn saturate() -> String {
        r#"// Chromatic saturate operation - adjust saturation
@group(0) @binding(0) var<storage, read> tensor: array<vec4<f32>>;
@group(0) @binding(1) var<storage, read_write> output: array<vec4<f32>>;
@group(0) @binding(2) var<uniform> alpha: f32;

@compute @workgroup_size(8, 8, 1)
fn chromatic_saturate(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x + id.y * 8u;
    let color = tensor[idx];

    // Calculate mean and scale distance from mean
    let mean = (color.r + color.g + color.b) / 3.0;
    let saturated = mean + alpha * (color.rgb - vec3<f32>(mean));

    output[idx] = vec4<f32>(clamp(saturated, vec3<f32>(0.0), vec3<f32>(1.0)), color.w);
}
"#
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = WGSLValidator::new();
        assert!(validator.show_warnings);
    }

    #[test]
    fn test_valid_wgsl() {
        let validator = WGSLValidator::new();
        let code = r#"
            @fragment
            fn main() -> @location(0) vec4<f32> {
                return vec4<f32>(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let result = validator.validate(code).unwrap();
        assert!(result.is_valid);
    }

    #[test]
    fn test_invalid_wgsl() {
        let validator = WGSLValidator::new();
        let code = "this is not valid WGSL code!!!";

        let result = validator.validate(code).unwrap();
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_chromatic_templates() {
        let validator = WGSLValidator::new();

        // Test all templates validate correctly
        for (name, template) in [
            ("mix", ChromaticTemplate::mix()),
            ("filter", ChromaticTemplate::filter()),
            ("complement", ChromaticTemplate::complement()),
            ("saturate", ChromaticTemplate::saturate()),
        ] {
            let result = validator.validate(&template).unwrap();
            assert!(result.is_valid, "Template '{}' should be valid", name);
        }
    }
}
