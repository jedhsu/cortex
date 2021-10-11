/// Matrix multiplication.
pub trait Mul {
    fn mul(
        left: &Self,
        right: &Self,
    );
}
