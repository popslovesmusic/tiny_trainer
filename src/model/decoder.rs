//! Transformer decoder layers with self and cross attention.

use ndarray::Array2;
use rand::{distributions::Uniform, rngs::StdRng};

use super::{attention::MultiHeadAttention, FeedForward, LayerNorm};

/// Decoder block with masked self-attention, encoder cross attention, and FFN.
#[derive(Debug, Clone)]
pub struct DecoderLayer {
    self_attn: MultiHeadAttention,
    norm1: LayerNorm,
    cross_attn: MultiHeadAttention,
    norm2: LayerNorm,
    feedforward: FeedForward,
    norm3: LayerNorm,
}

impl DecoderLayer {
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
            cross_attn: MultiHeadAttention::new(d_model, nhead, rng, dist),
            norm2: LayerNorm::new(d_model),
            feedforward: FeedForward::new(d_model, dim_feedforward, rng, dist),
            norm3: LayerNorm::new(d_model),
        }
    }

    pub fn forward(
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

    pub fn num_parameters(&self) -> usize {
        self.self_attn.num_parameters()
            + self.cross_attn.num_parameters()
            + self.feedforward.num_parameters()
            + self.norm1.num_parameters()
            + self.norm2.num_parameters()
            + self.norm3.num_parameters()
    }
}
