/// Operators are categorized into:
///
/// new             (...) -> Ten            Constructors
/// mold            Ten -> Ten              Geometric Morphisms (operates purely on "extrinsic container") TODO make rigorous
/// apply           Ten -> Ten              Applicative Morphisms (Maps from type to type intrinsically as well as extrinsically)
/// reduce          Ten > Scalar            Reductions (Maps from type to reduced axis type or a scalar) TODO clarify / separate
/// split           Ten -> (Ten, Ten)       One to many
/// merge           (Ten, Ten) -> Ten       Many to one
