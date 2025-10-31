//! Encoder stack placeholder

use super::attention::MultiHeadAttention;

pub struct EncoderLayer {
    pub attn: MultiHeadAttention,
}

impl EncoderLayer {
    pub fn new(d_model: usize, nhead: usize) -> Self {
        Self {
            attn: MultiHeadAttention::new(d_model, nhead),
        }
    }

    pub fn forward(&self) {
        self.attn.forward();
        println!("encoder layer forward()");
    }
}

pub struct EncoderStack {
    pub layers: Vec<EncoderLayer>,
}

impl EncoderStack {
    pub fn new(num_layers: usize, d_model: usize, nhead: usize) -> Self {
        Self {
            layers: (0..num_layers)
                .map(|_| EncoderLayer::new(d_model, nhead))
                .collect(),
        }
    }
}
