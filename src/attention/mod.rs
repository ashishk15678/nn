/// This module implements the attention mechanism from the 2017 paper "Attention Is All You Need".

/// Struct representing the scaled dot-product attention mechanism.
pub struct ScaledDotProductAttention {
    pub d_k: f64,
}

impl ScaledDotProductAttention {
    /// Creates a new instance of ScaledDotProductAttention.
    ///
    /// # Arguments
    ///
    /// * `d_k` - The dimension of the key vectors.
    pub fn new(d_k: f64) -> Self {
        Self { d_k }
    }

    /// Computes the attention scores.
    ///
    /// # Arguments
    ///
    /// * `query` - The query matrix.
    /// * `key` - The key matrix.
    /// * `value` - The value matrix.
    ///
    /// # Returns
    ///
    /// The attention output matrix.
    pub fn compute_attention(
        &self,
        query: &Vec<Vec<f64>>,
        key: &Vec<Vec<f64>>,
        value: &Vec<Vec<f64>>,
    ) -> Vec<Vec<f64>> {
        let scores = self.scaled_dot_product(query, key);
        let attention_weights = self.softmax(&scores);
        self.apply_attention(&attention_weights, value)
    }

    /// Computes the scaled dot-product of the query and key matrices.
    ///
    /// # Arguments
    ///
    /// * `query` - The query matrix.
    /// * `key` - The key matrix.
    ///
    /// # Returns
    ///
    /// The scaled dot-product matrix.
    fn scaled_dot_product(&self, query: &Vec<Vec<f64>>, key: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let mut product = vec![vec![0.0; key[0].len()]; query.len()];
        for i in 0..query.len() {
            for j in 0..key[0].len() {
                for k in 0..key.len() {
                    product[i][j] += query[i][k] * key[k][j];
                }
                product[i][j] /= self.d_k.sqrt();
            }
        }
        product
    }

    /// Applies the softmax function to the input matrix.
    ///
    /// # Arguments
    ///
    /// * `matrix` - The input matrix.
    ///
    /// # Returns
    ///
    /// The matrix after applying the softmax function.
    fn softmax(&self, matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let mut softmaxed = vec![vec![0.0; matrix[0].len()]; matrix.len()];
        for i in 0..matrix.len() {
            let mut sum = 0.0;
            for j in 0..matrix[0].len() {
                softmaxed[i][j] = matrix[i][j].exp();
                sum += softmaxed[i][j];
            }
            for j in 0..matrix[0].len() {
                softmaxed[i][j] /= sum;
            }
        }
        softmaxed
    }

    /// Applies the attention weights to the value matrix.
    ///
    /// # Arguments
    ///
    /// * `attention_weights` - The attention weights matrix.
    /// * `value` - The value matrix.
    ///
    /// # Returns
    ///
    /// The output matrix after applying the attention weights.
    fn apply_attention(
        &self,
        attention_weights: &Vec<Vec<f64>>,
        value: &Vec<Vec<f64>>,
    ) -> Vec<Vec<f64>> {
        let mut output = vec![vec![0.0; value[0].len()]; attention_weights.len()];
        for i in 0..attention_weights.len() {
            for j in 0..value[0].len() {
                for k in 0..value.len() {
                    output[i][j] += attention_weights[i][k] * value[k][j];
                }
            }
        }
        output
    }
}
