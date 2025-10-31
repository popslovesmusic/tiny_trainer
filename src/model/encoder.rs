//! Transformer encoder layers for the WGSL model.

use ndarray::Array2;
use rand::{distributions::Uniform, rngs::StdRng};

use super::{attention::MultiHeadAttention, FeedForward, LayerNorm};

/// Single encoder block consisting of self-attention and a feed-forward network.
#[derive(Debug, Clone)]
pub struct EncoderLayer {
    self_attn: MultiHeadAttention,
    norm1: LayerNorm,
    feedforward: FeedForward,
    norm2: LayerNorm,
}

impl EncoderLayer {
    pub fn new(
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

    pub fn forward(&self, x: &Array2<f32>, mask: Option<&Array2<f32>>) -> Array2<f32> {
        let attn_output = self.self_attn.forward(x, x, x, mask);
        let residual1 = x + &attn_output;
        let normed1 = self.norm1.forward(&residual1);
        let ff_output = self.feedforward.forward(&normed1);
        let residual2 = normed1 + &ff_output;
        self.norm2.forward(&residual2)
    }

    pub fn num_parameters(&self) -> usize {
        self.self_attn.num_parameters()
            + self.feedforward.num_parameters()
            + self.norm1.num_parameters()
            + self.norm2.num_parameters()
    }
}
