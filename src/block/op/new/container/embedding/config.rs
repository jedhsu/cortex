/// Configuration option for an embedding layer.
#[derive(Debug, Clone, Copy)]
pub struct EmbeddingConfig {
    pub sparse: bool,
    pub scale_grad_by_freq: bool,
    pub ws_init: super::Init,
    pub padding_idx: i64,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        EmbeddingConfig {
            sparse: false,
            scale_grad_by_freq: false,
            ws_init: super::Init::Randn {
                mean: 0.,
                stdev: 1.,
            },
            padding_idx: -1,
        }
    }
}
