/// Operators are categorized into:
///
/// new             (...) -> Ten            Constructors
/// morph           Ten -> Ten              Morphisms (Maps from type to type)
/// reduce          Ten > Scalar            Reductions (Maps from type to reduced axis type or a scalar) TODO clarify / separate
/// split           Ten -> (Ten, Ten)       One to many
/// merge           (Ten, Ten) -> Ten       Many to one
