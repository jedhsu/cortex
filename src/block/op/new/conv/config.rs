#[derive(Debug, Clone, Copy)]
pub struct ConvConfigND<ND> {
    pub stride: ND,
    pub padding: ND,
    pub dilation: ND,
    pub groups: i64,
    pub bias: bool,

    pub ws_init: super::Init,
    pub bs_init: super::Init,
}
