//! WGSL tokenization and vocabulary management
//!
//! Provides specialized tokenization for WGSL (WebGPU Shading Language) syntax

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Special tokens used in the vocabulary
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecialToken {
    Padding,
    Unknown,
    StartOfSequence,
    EndOfSequence,
}

impl SpecialToken {
    pub fn as_str(&self) -> &'static str {
        match self {
            SpecialToken::Padding => "<pad>",
            SpecialToken::Unknown => "<unk>",
            SpecialToken::StartOfSequence => "<sos>",
            SpecialToken::EndOfSequence => "<eos>",
        }
    }

    pub fn token_id(&self) -> usize {
        match self {
            SpecialToken::Padding => 0,
            SpecialToken::Unknown => 1,
            SpecialToken::StartOfSequence => 2,
            SpecialToken::EndOfSequence => 3,
        }
    }
}

/// WGSL-specialized tokenizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WGSLTokenizer {
    /// Token to ID mapping (string to index)
    pub vocab: HashMap<String, usize>,
    /// ID to token mapping (index to string)
    pub reverse_vocab: HashMap<usize, String>,
    /// Next available token ID
    next_id: usize,
    /// Maximum sequence length
    pub max_length: usize,
    /// Convert to lowercase
    pub lowercase: bool,
    /// WGSL-specific regex patterns
    #[serde(skip)]
    patterns: WGSLPatterns,
}

/// Compiled regex patterns for WGSL tokenization
#[derive(Debug, Clone)]
struct WGSLPatterns {
    /// WGSL keywords
    keyword: Regex,
    /// Type specifiers (vec3<f32>, etc.)
    type_spec: Regex,
    /// Attribute annotations (@compute, @fragment, etc.)
    attribute: Regex,
    /// Identifiers
    identifier: Regex,
    /// Numbers
    number: Regex,
    /// Operators
    operator: Regex,
    /// Punctuation
    punctuation: Regex,
}

impl Default for WGSLPatterns {
    fn default() -> Self {
        Self {
            // WGSL keywords
            keyword: Regex::new(
                r"\b(fn|var|let|const|struct|type|if|else|for|while|loop|break|continue|return|switch|case|default|discard|@compute|@fragment|@vertex|@group|@binding|@location|@builtin|@workgroup_size|@stage|@size|@align|@interpolate)\b"
            ).unwrap(),
            // Type specifiers: vec2<f32>, mat4x4<f32>, texture_2d<f32>, etc.
            type_spec: Regex::new(
                r"\b(vec[234]|mat[234]x[234]|array|texture_[123]d|texture_cube|texture_2d_array|texture_storage_[123]d|sampler|sampler_comparison|atomic|ptr)<[^>]+>"
            ).unwrap(),
            // Built-in types
            attribute: Regex::new(
                r"@(compute|fragment|vertex|group|binding|location|builtin|workgroup_size|stage|size|align|interpolate)"
            ).unwrap(),
            // Identifiers
            identifier: Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
            // Numbers (int, float, hex)
            number: Regex::new(r"0x[0-9a-fA-F]+|[0-9]+\.?[0-9]*([eE][+-]?[0-9]+)?[fu]?").unwrap(),
            // Operators
            operator: Regex::new(r"[+\-*/%&|^<>=!~]+|<<|>>|&&|\|\||==|!=|<=|>=|->").unwrap(),
            // Punctuation
            punctuation: Regex::new(r"[(){}\[\];:,.]").unwrap(),
        }
    }
}

impl WGSLTokenizer {
    /// Create a new WGSL tokenizer
    pub fn new(max_length: usize, lowercase: bool) -> Self {
        let mut tokenizer = WGSLTokenizer {
            vocab: HashMap::new(),
            reverse_vocab: HashMap::new(),
            next_id: 4, // Reserve 0-3 for special tokens
            max_length,
            lowercase,
            patterns: WGSLPatterns::default(),
        };

        // Add special tokens
        for special in &[
            SpecialToken::Padding,
            SpecialToken::Unknown,
            SpecialToken::StartOfSequence,
            SpecialToken::EndOfSequence,
        ] {
            let token = special.as_str().to_string();
            let id = special.token_id();
            tokenizer.vocab.insert(token.clone(), id);
            tokenizer.reverse_vocab.insert(id, token);
        }

        tokenizer
    }

    /// Tokenize WGSL code into tokens
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        let text = if self.lowercase {
            text.to_lowercase()
        } else {
            text.to_string()
        };

        let mut tokens = Vec::new();
        let mut pos = 0;

        while pos < text.len() {
            let remaining = &text[pos..];

            // Skip whitespace
            if let Some(ch) = remaining.chars().next() {
                if ch.is_whitespace() {
                    pos += ch.len_utf8();
                    continue;
                }
            }

            // Try matching patterns in order of priority
            let mut matched = false;

            // 1. Type specifiers (highest priority for WGSL)
            if let Some(mat) = self.patterns.type_spec.find(remaining) {
                if mat.start() == 0 {
                    tokens.push(mat.as_str().to_string());
                    pos += mat.end();
                    matched = true;
                    continue;
                }
            }

            // 2. Attributes
            if let Some(mat) = self.patterns.attribute.find(remaining) {
                if mat.start() == 0 {
                    tokens.push(mat.as_str().to_string());
                    pos += mat.end();
                    matched = true;
                    continue;
                }
            }

            // 3. Keywords
            if let Some(mat) = self.patterns.keyword.find(remaining) {
                if mat.start() == 0 {
                    tokens.push(mat.as_str().to_string());
                    pos += mat.end();
                    matched = true;
                    continue;
                }
            }

            // 4. Numbers
            if let Some(mat) = self.patterns.number.find(remaining) {
                if mat.start() == 0 {
                    tokens.push(mat.as_str().to_string());
                    pos += mat.end();
                    matched = true;
                    continue;
                }
            }

            // 5. Operators
            if let Some(mat) = self.patterns.operator.find(remaining) {
                if mat.start() == 0 {
                    tokens.push(mat.as_str().to_string());
                    pos += mat.end();
                    matched = true;
                    continue;
                }
            }

            // 6. Punctuation
            if let Some(mat) = self.patterns.punctuation.find(remaining) {
                if mat.start() == 0 {
                    tokens.push(mat.as_str().to_string());
                    pos += mat.end();
                    matched = true;
                    continue;
                }
            }

            // 7. Identifiers
            if let Some(mat) = self.patterns.identifier.find(remaining) {
                if mat.start() == 0 {
                    tokens.push(mat.as_str().to_string());
                    pos += mat.end();
                    matched = true;
                    continue;
                }
            }

            // If no pattern matched, skip this character
            if !matched {
                if let Some(ch) = remaining.chars().next() {
                    pos += ch.len_utf8();
                }
            }
        }

        tokens
    }

    /// Build vocabulary from training texts
    pub fn fit<S: AsRef<str>>(&mut self, texts: &[S], min_freq: usize) {
        // Count token frequencies
        let mut freq_map: HashMap<String, usize> = HashMap::new();

        for text in texts {
            let tokens = self.tokenize(text.as_ref());
            for token in tokens {
                *freq_map.entry(token).or_insert(0) += 1;
            }
        }

        // Add tokens that meet minimum frequency
        for (token, freq) in freq_map {
            if freq >= min_freq && !self.vocab.contains_key(&token) {
                let id = self.next_id;
                self.vocab.insert(token.clone(), id);
                self.reverse_vocab.insert(id, token);
                self.next_id += 1;
            }
        }
    }

    /// Encode tokens to IDs
    pub fn encode(&self, tokens: &[String]) -> Vec<usize> {
        tokens
            .iter()
            .map(|token| {
                self.vocab
                    .get(token)
                    .copied()
                    .unwrap_or(SpecialToken::Unknown.token_id())
            })
            .collect()
    }

    /// Encode text directly to IDs
    pub fn encode_text(&self, text: &str) -> Vec<usize> {
        let tokens = self.tokenize(text);
        self.encode(&tokens)
    }

    /// Decode IDs back to tokens
    pub fn decode(&self, ids: &[usize]) -> Vec<String> {
        ids.iter()
            .filter_map(|&id| self.reverse_vocab.get(&id).cloned())
            .collect()
    }

    /// Decode IDs to text
    pub fn decode_to_text(&self, ids: &[usize]) -> String {
        let tokens = self.decode(ids);
        tokens.join(" ")
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }

    /// Save tokenizer to JSON
    pub fn save<P: AsRef<Path>>(&self, path: P) -> crate::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load tokenizer from JSON
    pub fn load<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let mut tokenizer: WGSLTokenizer = serde_json::from_str(&json)?;
        tokenizer.patterns = WGSLPatterns::default();
        Ok(tokenizer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_tokens() {
        assert_eq!(SpecialToken::Padding.as_str(), "<pad>");
        assert_eq!(SpecialToken::Unknown.token_id(), 1);
    }

    #[test]
    fn test_tokenizer_creation() {
        let tokenizer = WGSLTokenizer::new(512, false);
        assert_eq!(tokenizer.vocab_size(), 4); // Only special tokens initially
        assert_eq!(tokenizer.max_length, 512);
    }

    #[test]
    fn test_wgsl_tokenization() {
        let tokenizer = WGSLTokenizer::new(512, false);
        let code = "fn main() -> vec4<f32> { return vec4<f32>(1.0, 0.0, 0.0, 1.0); }";
        let tokens = tokenizer.tokenize(code);

        assert!(tokens.contains(&"fn".to_string()));
        assert!(tokens.contains(&"main".to_string()));
        assert!(tokens.contains(&"vec4<f32>".to_string()));
        assert!(tokens.contains(&"return".to_string()));
        assert!(tokens.contains(&"1.0".to_string()));
    }

    #[test]
    fn test_attribute_tokenization() {
        let tokenizer = WGSLTokenizer::new(512, false);
        let code = "@compute @workgroup_size(8, 8, 1)";
        let tokens = tokenizer.tokenize(code);

        assert!(tokens.contains(&"@compute".to_string()));
        assert!(tokens.contains(&"@workgroup_size".to_string()));
    }

    #[test]
    fn test_fit_and_encode() {
        let mut tokenizer = WGSLTokenizer::new(512, false);
        let texts = vec!["fn test() { }", "var x: f32 = 1.0;"];

        tokenizer.fit(&texts, 1);

        let tokens = tokenizer.tokenize("fn test");
        let ids = tokenizer.encode(&tokens);

        assert!(ids.len() > 0);
        assert_ne!(ids[0], SpecialToken::Unknown.token_id());

        // Decode back
        let decoded = tokenizer.decode(&ids);
        assert_eq!(decoded, tokens);
    }

    #[test]
    fn test_encode_decode_text() {
        let mut tokenizer = WGSLTokenizer::new(512, false);
        let text = "fn main() {}";

        tokenizer.fit(&[text], 1);

        let ids = tokenizer.encode_text(text);
        let decoded = tokenizer.decode_to_text(&ids);

        // Should be similar (spaces might differ)
        assert!(decoded.contains("fn"));
        assert!(decoded.contains("main"));
    }
}
