/// Equivalences for Conv
impl Equiv for Conv {
    fn equiv1() -> Equiv<Conv, Conv> {
        Let(
            Conv(s, p, c, ScalarMul(𝚇, 𝑤), 𝙮),
            Conv(s, p, c, 𝚇, ScalarMul(𝙮, 𝑤)),
        )
    }

    fn equiv2() -> Equiv<Conv, Conv> {
        Let(
            ScalarMul(Conv(s, p, Anone, 𝚇, 𝙮), w),
            Conv(s, p, Anone, ScalarMul(𝚇, w), 𝙮),
        );
    }

    fn equiv3() -> Equiv<Conv, Conv> {
        Let(
            Conv(s, p, Anone, 𝚇, ElAdd(𝙮, 𝒁)),
            ElAdd(Conv(s, p, Anone, 𝚇, 𝙮), Conv(s, p, Anone, 𝚇, 𝒁)),
        );
    }

    fn equiv4() -> Equiv<Conv, Conv> {
        Let(
            Conv(s, p, Anone, ElAdd(𝚇, 𝙮), 𝒁),
            (Conv(s, p, Anone, 𝚇, 𝒁), Conv(s, p, Anone, 𝙮, 𝒁)),
        )
    }

    fn equiv5() -> Equiv<Conv, Conv> {
        Let(Conv(s, Psame, c, 𝚇, 𝙮), Conv(s, Psame, c, 𝚇, Enlarge(k, 𝙮)));
    }

    fn equiv6() -> Equiv<Conv> {
        Let(Conv(s, p, Arelu, 𝚇, 𝙮), Relu(Conv(s, p, Anone, 𝚇, 𝙮)));
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn test_equiv1() -> Result<()> {}
}

///// Sick
//macro_rules! 𝙵𝚗 {
//}

//macro_rules! 𝚃𝚎𝚗 {
//}

///// semantics for the dependently typed tensor.
//macro_rules! TensorType {

//}
///// Macro for params in ml and all the boilerplate!

//𝚃𝚎𝚗!
//(A ==>
//[1 2 3
// 4 5 6
// 7 8 9]
//);

///// Can create a dependent type check!

//// ∀s, p, x, k. Conv(s, p, Anone, x, Cpool(k)) = poolavg(k, s, p, x)
////
//// ∀k, x . Conv(1, Psame, Anone, x, IConv(k)) = x
