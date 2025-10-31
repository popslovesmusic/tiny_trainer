//! Multi-head attention implementation used by the WGSL transformer.

use ndarray::{s, Array1, Array2};
use rand::{distributions::Uniform, rngs::StdRng, Rng};

use super::softmax_vec;

/// Multi-head scaled dot-product attention.
#[derive(Debug, Clone)]
pub struct MultiHeadAttention {
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
}

impl MultiHeadAttention {
    /// Create a new attention module with Xavier-like random initialisation.
    pub fn new(d_model: usize, nhead: usize, rng: &mut StdRng, dist: Uniform<f32>) -> Self {
        assert!(d_model % nhead == 0, "d_model must be divisible by nhead");

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
            head_dim: d_model / nhead,
            w_q,
            w_k,
            w_v,
            w_o,
            b_q,
            b_k,
            b_v,
            b_o,
        }
    }

    /// Forward pass of the attention module.
    pub fn forward(
        &self,
        query: &Array2<f32>,
        key: &Array2<f32>,
        value: &Array2<f32>,
        mask: Option<&Array2<f32>>,
    ) -> Array2<f32> {
        let q = query.dot(&self.w_q) + &self.b_q;
        let k = key.dot(&self.w_k) + &self.b_k;
        let v = value.dot(&self.w_v) + &self.b_v;

        let query_len = q.nrows();
        let key_len = k.nrows();

        let mut weights = Array2::<f32>::zeros((query_len, key_len));
        let mut context = Array2::<f32>::zeros((query_len, self.d_model));

        for head in 0..self.nhead {
            let start = head * self.head_dim;
            let end = start + self.head_dim;

            let q_head = q.slice(s![.., start..end]).to_owned();
            let k_head = k.slice(s![.., start..end]).to_owned();
            let v_head = v.slice(s![.., start..end]).to_owned();

            for i in 0..query_len {
                for j in 0..key_len {
                    let mut score = 0.0f32;
                    for d in 0..self.head_dim {
                        score += q_head[[i, d]] * k_head[[j, d]];
                    }
                    score /= (self.head_dim as f32).sqrt();
                    weights[[i, j]] = score;
                }
            }

            if let Some(mask) = mask {
                for i in 0..query_len {
                    for j in 0..key_len {
                        weights[[i, j]] += mask[[i, j]];
                    }
                }
            }

            for i in 0..query_len {
                let mut logits = Vec::with_capacity(key_len);
                for j in 0..key_len {
                    logits.push(weights[[i, j]]);
                }
                let softmax = softmax_vec(logits);
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

    /// Number of trainable parameters contained in this module.
    pub fn num_parameters(&self) -> usize {
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
