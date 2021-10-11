/// An embedding layer.
///
/// An embedding layer acts as a simple lookup table that stores embeddings.
/// This is commonly used to store word embeddings.
use crate::Tensor;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct Embedding {
    pub ws: Tensor,
    config: EmbeddingConfig,
}

pub fn embedding<'a, T: Borrow<super::Path<'a>>>(
    vs: T,
    num_embeddings: i64,
    embedding_dim: i64,
    config: EmbeddingConfig,
) -> Embedding {
    let vs = vs.borrow();
    Embedding {
        ws: vs.var("weight", &[num_embeddings, embedding_dim], config.ws_init),
        config,
    }
}

impl super::module::Module for Embedding {
    fn forward(
        &self,
        xs: &Tensor,
    ) -> Tensor {
        Tensor::embedding(
            &self.ws,
            xs,
            self.config.padding_idx,
            self.config.scale_grad_by_freq,
            self.config.sparse,
        )
    }
}
