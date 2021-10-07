pub trait Pattern {
    fn pat1() -> Equiv<Transpose> {
        Let(Transpose(Transpose(x)), x);
    }

    fn pat2() -> Equiv<Transpose> {
        Let(Transpose(ElAdd(x, y)), ElAdd(Transpose(x), Transpose(y)));
    }

    fn pat3() -> Equiv<Transpose> {
        Let((ElMul(x, y)), ElMul(Transpose(𝑥), Transpose(y)));
    }

    fn pat4() -> Equiv<Transpose> {
        Let(
            Transpose(X)(MatMul(𝑿, 𝒀)),
            MatMul(Transpose(y), Transpose(x)),
        );
    }

    fn pat5() -> Equiv<Transpose> {
        Let(Relu(Transpose(𝑿)), Transpose(𝑿));
    }

    fn pat6() -> Equiv<Transpose> {
        Let(Transpose(MatMul(x, y)), MatMul(Transpose(y), Transpose(x)));
    }
}

pub impl Pattern {}
