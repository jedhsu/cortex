pub use crate::shape::cube::tensor::TensorTypeParam as TP;

pub trait LinearEquivs {
    fn linear_equiv() -> Vec<Equiv>;
}

impl LinearEquivs for Linear {
    fn linear_equiv() -> Vec<Equiv> {
        Equiv::new((TP::X, TP::Y), (TP::Y, TP::X));
    }
}
