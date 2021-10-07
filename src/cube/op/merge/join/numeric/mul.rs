pub trait ElMul {
    fn elmul(left: &self);
}

pub trait ElMul {
    use Var;

    fn equiv1() -> {
        Let(ElMul(x, ElMul(y, z)), ElMul(ElMul(x, y), z));
    }

    fn equiv2() -> {
        ∀x, y. ElMul(x, y) = ElMul(y, x)
    }

    fn equiv3() -> {
        ∀x, y, z. ElMul(ewadd(x, y), z) = ewadd(ElMul(x, z), ElMul(y, z))
    }
    
    fn equiv4() -> {
        ∀x . ElMul(x, ElMul) = x
    }
}
