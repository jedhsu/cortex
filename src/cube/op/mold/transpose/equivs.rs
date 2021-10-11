pub trait Transpose {
    fn equiv1() -> Equiv<Transpose>;
    fn equiv2() -> Equiv<Transpose>;
    fn equiv3() -> Equiv<Transpose>;
    fn equiv4() -> Equiv<Transpose>;
    fn equiv5() -> Equiv<Transpose>;
    fn equiv2() -> Equiv<Transpose>;
}

impl Equiv for Transpose {
    fn equiv1() -> Equiv<Transpose> {
        Let(Transpose(Transpose(x)), x);
    }

    fn equiv2() -> Equiv<Transpose> {
        Let(Transpose(ElAdd(x, y)), ElAdd(Transpose(x), Transpose(y)));
    }

    fn equiv3() -> Equiv<Transpose> {
        Let((ElMul(x, y)), ElMul(Transpose(𝑥), Transpose(y)));
    }

    fn equiv4() -> Equiv<Transpose> {
        Let(
            Transpose(X)(MatMul(𝑿, 𝒀)),
            MatMul(Transpose(y), Transpose(x)),
        );
    }

    fn equiv5() -> Equiv<Transpose> {
        Let(Relu(Transpose(𝑿)), Transpose(𝑿));
    }

    fn equiv6() -> Equiv<Transpose> {
        Let(Transpose(MatMul(x, y)), MatMul(Transpose(y), Transpose(x)));
    }
}
