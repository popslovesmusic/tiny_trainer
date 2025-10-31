//! Neural network models for WGSL code generation
//!
//! Implements an encoder-decoder transformer tailored for WGSL token sequences.

pub mod attention;
pub mod decoder;
pub mod encoder;

use crate::config::ModelConfig;
use crate::tokenizer::SpecialToken;
use ndarray::{s, Array1, Array2};
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use decoder::DecoderLayer;
use encoder::EncoderLayer;

const DEFAULT_MAX_SEQ_LEN: usize = 512;
const DEFAULT_DIM_FEEDFORWARD: usize = 2048;

/// Model architecture types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelArchitecture {
    Transformer,
    LSTM,
}

/// Neural network model for code generation
#[derive(Debug, Clone)]
pub struct CodeGenerationModel {
    pub architecture: ModelArchitecture,
    pub vocab_size: usize,
    pub d_model: usize,
    pub nhead: usize,
    pub num_layers: usize,
    pub max_seq_len: usize,
    pub dim_feedforward: usize,
    transformer: Option<Transformer>,
}

impl CodeGenerationModel {
    /// Create a new model with the given configuration
    pub fn new(
        architecture: ModelArchitecture,
        vocab_size: usize,
        d_model: usize,
        nhead: usize,
        num_layers: usize,
        dim_feedforward: Option<usize>,
        max_seq_len: Option<usize>,
    ) -> Self {
        let transformer = match architecture {
            ModelArchitecture::Transformer => Some(Transformer::new(
                vocab_size,
                d_model,
                nhead,
                num_layers,
                max_seq_len.unwrap_or(DEFAULT_MAX_SEQ_LEN),
                dim_feedforward.unwrap_or(DEFAULT_DIM_FEEDFORWARD),
            )),
            ModelArchitecture::LSTM => None,
        };

        Self {
            architecture,
            vocab_size,
            d_model,
            nhead,
            num_layers,
            max_seq_len: max_seq_len.unwrap_or(DEFAULT_MAX_SEQ_LEN),
            dim_feedforward: dim_feedforward.unwrap_or(DEFAULT_DIM_FEEDFORWARD),
            transformer,
        }
    }

    /// Create a model from a [`ModelConfig`], applying production defaults when
    /// configuration values are absent.
    pub fn from_model_config(vocab_size: usize, config: &ModelConfig) -> Self {
        let architecture = match config.architecture.to_lowercase().as_str() {
            "transformer" => ModelArchitecture::Transformer,
            "lstm" => ModelArchitecture::LSTM,
            other => {
                tracing::warn!(
                    "Unknown architecture '{}' in model config; defaulting to transformer",
                    other
                );
                ModelArchitecture::Transformer
            }
        };

        Self::new(
            architecture,
            vocab_size,
            config.d_model,
            config.nhead,
            config.num_layers,
            Some(config.dim_feedforward),
            Some(config.max_seq_len),
        )
    }

    /// Forward pass through the underlying model.
    ///
    /// For the transformer, this uses the input tokens for both the encoder and
    /// decoder streams with a prepended start-of-sequence token for the decoder.
    pub fn forward(&self, input_ids: &[usize]) -> Vec<f32> {
        match self.architecture {
            ModelArchitecture::Transformer => {
                let transformer = self
                    .transformer
                    .as_ref()
                    .expect("transformer should be initialized");

                let mut decoder_input = Vec::with_capacity(input_ids.len() + 1);
                decoder_input.push(SpecialToken::StartOfSequence.token_id());
                decoder_input.extend_from_slice(input_ids);

                let logits = transformer.forward(input_ids, &decoder_input);
                let last_row = logits.row(logits.nrows() - 1);
                last_row.to_vec()
            }
            ModelArchitecture::LSTM => vec![0.0; self.vocab_size],
        }
    }

    /// Get number of parameters
    pub fn num_parameters(&self) -> usize {
        match self.architecture {
            ModelArchitecture::Transformer => self
                .transformer
                .as_ref()
                .map(|t| t.num_parameters())
                .unwrap_or(0),
            ModelArchitecture::LSTM => 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Transformer {
    vocab_size: usize,
    d_model: usize,
    nhead: usize,
    num_layers: usize,
    max_seq_len: usize,
    dim_feedforward: usize,
    token_embedding: Array2<f32>,
    positional_encoding: Array2<f32>,
    encoder_layers: Vec<EncoderLayer>,
    decoder_layers: Vec<DecoderLayer>,
    final_linear_weight: Array2<f32>,
    final_linear_bias: Array1<f32>,
}

impl Transformer {
    fn new(
        vocab_size: usize,
        d_model: usize,
        nhead: usize,
        num_layers: usize,
        max_seq_len: usize,
        dim_feedforward: usize,
    ) -> Self {
        assert!(d_model % nhead == 0, "d_model must be divisible by nhead");

        let mut rng = StdRng::seed_from_u64(42);
        let dist = Uniform::new(-0.1f32, 0.1f32);

        let token_embedding = Array2::from_shape_fn((vocab_size, d_model), |_| rng.sample(dist));
        let positional_encoding = Self::create_positional_encoding(max_seq_len, d_model);

        let mut encoder_layers = Vec::with_capacity(num_layers);
        let mut decoder_layers = Vec::with_capacity(num_layers);

        for _ in 0..num_layers {
            encoder_layers.push(EncoderLayer::new(
                d_model,
                nhead,
                dim_feedforward,
                &mut rng,
                dist,
            ));
            decoder_layers.push(DecoderLayer::new(
                d_model,
                nhead,
                dim_feedforward,
                &mut rng,
                dist,
            ));
        }

        let final_linear_weight =
            Array2::from_shape_fn((d_model, vocab_size), |_| rng.sample(dist));
        let final_linear_bias = Array1::from_shape_fn(vocab_size, |_| rng.sample(dist));

        Self {
            vocab_size,
            d_model,
            nhead,
            num_layers,
            max_seq_len,
            dim_feedforward,
            token_embedding,
            positional_encoding,
            encoder_layers,
            decoder_layers,
            final_linear_weight,
            final_linear_bias,
        }
    }

    fn forward(&self, encoder_input: &[usize], decoder_input: &[usize]) -> Array2<f32> {
        let encoder_ids = self.sanitize_ids(encoder_input);
        let decoder_ids = self.sanitize_ids(decoder_input);

        let mut encoder_states = self.embed(&encoder_ids);
        let mut decoder_states = self.embed(&decoder_ids);

        let encoder_self_mask = self.self_padding_mask(&encoder_ids);
        let decoder_self_mask = self.self_padding_mask(&decoder_ids);
        let look_ahead = self.look_ahead_mask(decoder_ids.len());
        let decoder_mask = self.combine_masks(&decoder_self_mask, &look_ahead);
        let cross_mask = self.cross_padding_mask(decoder_ids.len(), &encoder_ids);

        for layer in &self.encoder_layers {
            encoder_states = layer.forward(&encoder_states, Some(&encoder_self_mask));
        }

        for layer in &self.decoder_layers {
            decoder_states = layer.forward(
                &decoder_states,
                &encoder_states,
                Some(&decoder_mask),
                Some(&cross_mask),
            );
        }

        decoder_states.dot(&self.final_linear_weight) + &self.final_linear_bias
    }

    fn embed(&self, input_ids: &[usize]) -> Array2<f32> {
        let seq_len = input_ids.len();
        let mut output = Array2::<f32>::zeros((seq_len, self.d_model));

        for (position, &token_id) in input_ids.iter().enumerate() {
            let token_vec = self.token_embedding.row(token_id);
            let pos_vec = self.positional_encoding.row(position % self.max_seq_len);
            let mut dest = output.slice_mut(s![position, ..]);
            dest.assign(&(&token_vec + &pos_vec));
        }

        output
    }

    fn sanitize_ids(&self, ids: &[usize]) -> Vec<usize> {
        ids.iter()
            .take(self.max_seq_len)
            .map(|&id| {
                if id < self.vocab_size {
                    id
                } else {
                    SpecialToken::Unknown.token_id()
                }
            })
            .collect()
    }

    fn self_padding_mask(&self, ids: &[usize]) -> Array2<f32> {
        let seq_len = ids.len();
        let mut mask = Array2::<f32>::zeros((seq_len, seq_len));
        for i in 0..seq_len {
            for j in 0..seq_len {
                if ids[j] == SpecialToken::Padding.token_id() {
                    mask[[i, j]] = f32::NEG_INFINITY;
                }
            }
        }
        mask
    }

    fn cross_padding_mask(&self, query_len: usize, encoder_ids: &[usize]) -> Array2<f32> {
        let mut mask = Array2::<f32>::zeros((query_len, encoder_ids.len()));
        for i in 0..query_len {
            for (j, &id) in encoder_ids.iter().enumerate() {
                if id == SpecialToken::Padding.token_id() {
                    mask[[i, j]] = f32::NEG_INFINITY;
                }
            }
        }
        mask
    }

    fn look_ahead_mask(&self, seq_len: usize) -> Array2<f32> {
        let mut mask = Array2::<f32>::zeros((seq_len, seq_len));
        for i in 0..seq_len {
            for j in 0..seq_len {
                if j > i {
                    mask[[i, j]] = f32::NEG_INFINITY;
                }
            }
        }
        mask
    }

    fn combine_masks(&self, a: &Array2<f32>, b: &Array2<f32>) -> Array2<f32> {
        assert_eq!(a.shape(), b.shape());
        a + b
    }

    fn create_positional_encoding(max_seq_len: usize, d_model: usize) -> Array2<f32> {
        let mut encoding = Array2::<f32>::zeros((max_seq_len, d_model));
        for pos in 0..max_seq_len {
            for i in 0..d_model {
                let angle = pos as f32 / 10000_f32.powf((2 * (i / 2)) as f32 / d_model as f32);
                if i % 2 == 0 {
                    encoding[[pos, i]] = angle.sin();
                } else {
                    encoding[[pos, i]] = angle.cos();
                }
            }
        }
        encoding
    }

    fn num_parameters(&self) -> usize {
        let mut total = self.token_embedding.len()
            + self.final_linear_weight.len()
            + self.final_linear_bias.len();
        for layer in &self.encoder_layers {
            total += layer.num_parameters();
        }
        for layer in &self.decoder_layers {
            total += layer.num_parameters();
        }
        total
    }
}

#[derive(Debug, Clone)]
pub(super) struct FeedForward {
    linear1: Linear,
    linear2: Linear,
}

impl FeedForward {
    pub(super) fn new(
        d_model: usize,
        hidden_dim: usize,
        rng: &mut StdRng,
        dist: Uniform<f32>,
    ) -> Self {
        Self {
            linear1: Linear::new(d_model, hidden_dim, rng, dist),
            linear2: Linear::new(hidden_dim, d_model, rng, dist),
        }
    }

    pub(super) fn forward(&self, x: &Array2<f32>) -> Array2<f32> {
        let mut hidden = self.linear1.forward(x);
        hidden.mapv_inplace(|v| v.max(0.0));
        self.linear2.forward(&hidden)
    }

    pub(super) fn num_parameters(&self) -> usize {
        self.linear1.num_parameters() + self.linear2.num_parameters()
    }
}

#[derive(Debug, Clone)]
struct Linear {
    weight: Array2<f32>,
    bias: Array1<f32>,
}

impl Linear {
    fn new(in_dim: usize, out_dim: usize, rng: &mut StdRng, dist: Uniform<f32>) -> Self {
        let weight = Array2::from_shape_fn((in_dim, out_dim), |_| rng.sample(dist));
        let bias = Array1::from_shape_fn(out_dim, |_| rng.sample(dist));
        Self { weight, bias }
    }

    fn forward(&self, x: &Array2<f32>) -> Array2<f32> {
        x.dot(&self.weight) + &self.bias
    }

    fn num_parameters(&self) -> usize {
        self.weight.len() + self.bias.len()
    }
}

#[derive(Debug, Clone)]
pub(super) struct LayerNorm {
    gamma: Array1<f32>,
    beta: Array1<f32>,
    eps: f32,
}

impl LayerNorm {
    pub(super) fn new(dim: usize) -> Self {
        Self {
            gamma: Array1::ones(dim),
            beta: Array1::zeros(dim),
            eps: 1e-5,
        }
    }

    pub(super) fn forward(&self, x: &Array2<f32>) -> Array2<f32> {
        let mut output = x.clone();
        for mut row in output.rows_mut() {
            let len = row.len();
            let mut mean = 0.0f32;
            for idx in 0..len {
                mean += row[idx];
            }
            mean /= len as f32;

            let mut variance = 0.0f32;
            for idx in 0..len {
                let diff = row[idx] - mean;
                variance += diff * diff;
            }
            variance /= len as f32;
            let denom = (variance + self.eps).sqrt();

            for idx in 0..len {
                let normalized = (row[idx] - mean) / denom;
                row[idx] = normalized * self.gamma[idx] + self.beta[idx];
            }
        }
        output
    }

    pub(super) fn num_parameters(&self) -> usize {
        self.gamma.len() + self.beta.len()
    }
}

pub(super) fn softmax_vec(mut values: Vec<f32>) -> Vec<f32> {
    if values.is_empty() {
        return values;
    }

    let mut max = f32::NEG_INFINITY;
    for &v in &values {
        if v.is_finite() && v > max {
            max = v;
        }
    }

    if !max.is_finite() {
        let uniform = 1.0 / values.len() as f32;
        for v in values.iter_mut() {
            *v = uniform;
        }
        return values;
    }

    let mut sum = 0.0f32;
    for v in values.iter_mut() {
        if v.is_finite() {
            *v = (*v - max).exp();
        } else {
            *v = 0.0;
        }
        sum += *v;
    }

    if sum == 0.0 {
        let uniform = 1.0 / values.len() as f32;
        for v in values.iter_mut() {
            *v = uniform;
        }
        return values;
    }

    for v in values.iter_mut() {
        *v /= sum;
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ModelConfig;

    #[test]
    fn test_model_creation() {
        let model =
            CodeGenerationModel::new(ModelArchitecture::Transformer, 10000, 512, 8, 6, None, None);
        assert_eq!(model.vocab_size, 10000);
        assert!(model.num_parameters() > 0);
    }

    #[test]
    fn test_forward_shape() {
        let model =
            CodeGenerationModel::new(ModelArchitecture::Transformer, 128, 512, 8, 2, None, None);
        let input = vec![5, 6, 7, 8];
        let logits = model.forward(&input);
        assert_eq!(logits.len(), model.vocab_size);
    }

    #[test]
    fn test_from_model_config() {
        let config = ModelConfig {
            architecture: "transformer".to_string(),
            d_model: 512,
            nhead: 8,
            num_layers: 6,
            dim_feedforward: 2048,
            dropout: 0.1,
            max_seq_len: 512,
        };

        let model = CodeGenerationModel::from_model_config(2048, &config);
        assert_eq!(model.d_model, 512);
        assert_eq!(model.dim_feedforward, 2048);
        assert_eq!(model.max_seq_len, 512);
    }
}
