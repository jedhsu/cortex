pub trait ElMul {
    fn elmul(left: &self);
}

/// Matrix multiplication.
pub trait Mul {
    fn mul(left: &Self, right: &Self);
}

impl Equiv for Mul {
    fn equiv1() -> Equiv<Self> {
        Transpose(Mul(𝐗, Mul(y, z)) = Mul(Mul(𝐗, y), z));
    }

    fn equiv2() -> Equiv<Self> {
        Let(Mul(𝐗, ElAdd(y, z)), (Mul(𝐗, y), Mul(_, z)));
    }

    fn equiv3() -> Equiv<Self> {
        Let(Mat(𝐗, IMul), 𝐗);
    }

    fn equiv4() -> Equiv<Self> {
        Let(Mat(Concat(1, 𝐗, z), Concat(0, y, w)) = equivd(Mul(𝐗, y), equivd(z, w)))
    }
}
