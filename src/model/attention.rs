//! Multi-head attention placeholder (no math yet)

pub struct MultiHeadAttention {
    pub d_model: usize,
    pub nhead: usize,
}

impl MultiHeadAttention {
    pub fn new(d_model: usize, nhead: usize) -> Self {
        Self { d_model, nhead }
    }

    pub fn forward(&self) {
        // TODO: add Q,K,V projection + matmul
        println!("attention forward()");
    }
}
