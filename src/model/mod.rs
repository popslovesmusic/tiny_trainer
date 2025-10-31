//! Neural network models for WGSL code generation
//!
//! Implements an encoder-decoder transformer tailored for WGSL token sequences.

use crate::config::ModelConfig;
use crate::tokenizer::SpecialToken;
use ndarray::{s, Array1, Array2};
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

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
struct EncoderLayer {
    self_attn: MultiHeadAttention,
    norm1: LayerNorm,
    feedforward: FeedForward,
    norm2: LayerNorm,
}

impl EncoderLayer {
    fn new(
        d_model: usize,
        nhead: usize,
        dim_feedforward: usize,
        rng: &mut StdRng,
        dist: Uniform<f32>,
    ) -> Self {
        Self {
            self_attn: MultiHeadAttention::new(d_model, nhead, rng, dist),
            norm1: LayerNorm::new(d_model),
            feedforward: FeedForward::new(d_model, dim_feedforward, rng, dist),
            norm2: LayerNorm::new(d_model),
        }
    }

    fn forward(&self, x: &Array2<f32>, mask: Option<&Array2<f32>>) -> Array2<f32> {
        let attn_output = self.self_attn.forward(x, x, x, mask);
        let residual1 = x + &attn_output;
        let normed1 = self.norm1.forward(&residual1);
        let ff_output = self.feedforward.forward(&normed1);
        let residual2 = normed1 + &ff_output;
        self.norm2.forward(&residual2)
    }

    fn num_parameters(&self) -> usize {
        self.self_attn.num_parameters()
            + self.feedforward.num_parameters()
            + self.norm1.num_parameters()
            + self.norm2.num_parameters()
    }
}

#[derive(Debug, Clone)]
struct DecoderLayer {
    self_attn: MultiHeadAttention,
    norm1: LayerNorm,
    cross_attn: MultiHeadAttention,
    norm2: LayerNorm,
    feedforward: FeedForward,
    norm3: LayerNorm,
}

impl DecoderLayer {
    fn new(
        d_model: usize,
        nhead: usize,
        dim_feedforward: usize,
        rng: &mut StdRng,
        dist: Uniform<f32>,
    ) -> Self {
        Self {
            self_attn: MultiHeadAttention::new(d_model, nhead, rng, dist),
            norm1: LayerNorm::new(d_model),
            cross_attn: MultiHeadAttention::new(d_model, nhead, rng, dist),
            norm2: LayerNorm::new(d_model),
            feedforward: FeedForward::new(d_model, dim_feedforward, rng, dist),
            norm3: LayerNorm::new(d_model),
        }
    }

    fn forward(
        &self,
        x: &Array2<f32>,
        encoder_states: &Array2<f32>,
        self_mask: Option<&Array2<f32>>,
        cross_mask: Option<&Array2<f32>>,
    ) -> Array2<f32> {
        let self_attn = self.self_attn.forward(x, x, x, self_mask);
        let residual1 = x + &self_attn;
        let normed1 = self.norm1.forward(&residual1);

        let cross_attn =
            self.cross_attn
                .forward(&normed1, encoder_states, encoder_states, cross_mask);
        let residual2 = normed1 + &cross_attn;
        let normed2 = self.norm2.forward(&residual2);

        let ff_output = self.feedforward.forward(&normed2);
        let residual3 = normed2 + &ff_output;
        self.norm3.forward(&residual3)
    }

    fn num_parameters(&self) -> usize {
        self.self_attn.num_parameters()
            + self.cross_attn.num_parameters()
            + self.feedforward.num_parameters()
            + self.norm1.num_parameters()
            + self.norm2.num_parameters()
            + self.norm3.num_parameters()
    }
}

#[derive(Debug, Clone)]
struct MultiHeadAttention {
    d_model: usize,
    nhead: usize,
    head_dim: usize,
    w_q: Array2<f32>,
    w_k: Array2<f32>,
    w_v: Array2<f32>,
    w_o: Array2<f32>,
    b_q: Array1<f32>,
    b_k: Array1<f32>,
    b_v: Array1<f32>,
    b_o: Array1<f32>,
    scale: f32,
}

impl MultiHeadAttention {
    fn new(d_model: usize, nhead: usize, rng: &mut StdRng, dist: Uniform<f32>) -> Self {
        let head_dim = d_model / nhead;
        let w_q = Array2::from_shape_fn((d_model, d_model), |_| rng.sample(dist));
        let w_k = Array2::from_shape_fn((d_model, d_model), |_| rng.sample(dist));
        let w_v = Array2::from_shape_fn((d_model, d_model), |_| rng.sample(dist));
        let w_o = Array2::from_shape_fn((d_model, d_model), |_| rng.sample(dist));
        let b_q = Array1::from_shape_fn(d_model, |_| rng.sample(dist));
        let b_k = Array1::from_shape_fn(d_model, |_| rng.sample(dist));
        let b_v = Array1::from_shape_fn(d_model, |_| rng.sample(dist));
        let b_o = Array1::from_shape_fn(d_model, |_| rng.sample(dist));

        Self {
            d_model,
            nhead,
            head_dim,
            w_q,
            w_k,
            w_v,
            w_o,
            b_q,
            b_k,
            b_v,
            b_o,
            scale: (head_dim as f32).sqrt(),
        }
    }

    fn forward(
        &self,
        query: &Array2<f32>,
        key: &Array2<f32>,
        value: &Array2<f32>,
        mask: Option<&Array2<f32>>,
    ) -> Array2<f32> {
        let query_proj = query.dot(&self.w_q) + &self.b_q;
        let key_proj = key.dot(&self.w_k) + &self.b_k;
        let value_proj = value.dot(&self.w_v) + &self.b_v;

        let query_len = query_proj.nrows();
        let key_len = key_proj.nrows();

        let mut context = Array2::<f32>::zeros((query_len, self.d_model));

        for head in 0..self.nhead {
            let start = head * self.head_dim;
            let end = start + self.head_dim;

            let q_head = query_proj.slice(s![.., start..end]);
            let k_head = key_proj.slice(s![.., start..end]);
            let v_head = value_proj.slice(s![.., start..end]);

            let mut weights = Array2::<f32>::zeros((query_len, key_len));
            for i in 0..query_len {
                let mut row = Vec::with_capacity(key_len);
                for j in 0..key_len {
                    let mut score = q_head.row(i).dot(&k_head.row(j));
                    score /= self.scale.max(1e-6);
                    if let Some(mask) = mask {
                        score += mask[[i, j]];
                    }
                    row.push(score);
                }
                let softmax = softmax_vec(row);
                for (j, value) in softmax.into_iter().enumerate() {
                    weights[[i, j]] = value;
                }
            }

            for i in 0..query_len {
                for j in 0..key_len {
                    let weight = weights[[i, j]];
                    if weight == 0.0 {
                        continue;
                    }
                    for d in 0..self.head_dim {
                        context[[i, start + d]] += weight * v_head[[j, d]];
                    }
                }
            }
        }

        context.dot(&self.w_o) + &self.b_o
    }

    fn num_parameters(&self) -> usize {
        self.w_q.len()
            + self.w_k.len()
            + self.w_v.len()
            + self.w_o.len()
            + self.b_q.len()
            + self.b_k.len()
            + self.b_v.len()
            + self.b_o.len()
    }
}

#[derive(Debug, Clone)]
struct FeedForward {
    linear1: Linear,
    linear2: Linear,
}

impl FeedForward {
    fn new(d_model: usize, hidden_dim: usize, rng: &mut StdRng, dist: Uniform<f32>) -> Self {
        Self {
            linear1: Linear::new(d_model, hidden_dim, rng, dist),
            linear2: Linear::new(hidden_dim, d_model, rng, dist),
        }
    }

    fn forward(&self, x: &Array2<f32>) -> Array2<f32> {
        let mut hidden = self.linear1.forward(x);
        hidden.mapv_inplace(|v| v.max(0.0));
        self.linear2.forward(&hidden)
    }

    fn num_parameters(&self) -> usize {
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
struct LayerNorm {
    gamma: Array1<f32>,
    beta: Array1<f32>,
    eps: f32,
}

impl LayerNorm {
    fn new(dim: usize) -> Self {
        Self {
            gamma: Array1::ones(dim),
            beta: Array1::zeros(dim),
            eps: 1e-5,
        }
    }

    fn forward(&self, x: &Array2<f32>) -> Array2<f32> {
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

    fn num_parameters(&self) -> usize {
        self.gamma.len() + self.beta.len()
    }
}

fn softmax_vec(mut values: Vec<f32>) -> Vec<f32> {
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
