//! Decoder stack placeholder

use super::attention::MultiHeadAttention;

pub struct DecoderLayer {
    pub self_attn: MultiHeadAttention,
    pub cross_attn: MultiHeadAttention,
}

impl DecoderLayer {
    pub fn new(d_model: usize, nhead: usize) -> Self {
        Self {
            self_attn: MultiHeadAttention::new(d_model, nhead),
            cross_attn: MultiHeadAttention::new(d_model, nhead),
        }
    }

    pub fn forward(&self) {
        println!("decoder layer forward()");
    }
}
