pub trait Relu: Activation {
    fn leaky_relu(xs: &Tensor) -> Tensor {
        xs.maximum(&(xs * 0.2))
    }
}
