/// The shape of a tensor.
///
/// This is the extrinsic geometry, notice how elements are not relevant.

pub struct _TwoShape_(u32, u32);

impl _TwoShape_ {
    fn new(
        &self,
        sizing: Vec<u32>,
    ) {
        super(Shape, self).__new__(
            tuple,
            sizing,
        )
    }

    @classmethod
    def create(
        cls,
        *size: int,
    ):
        return cls([*size])

impl Display for _TwoShape_ {
    fn repr(&self) -> &str {
        "[" + " | ".join([str(part) for part in self]) + "]"
    }


class Test:
    @staticmethod
    def vector():
        return Shape.create(5)

    @staticmethod
    def grid():
        return Shape.create(5, 5)

    @staticmethod
    def threetope():
        return Shape.create(5, 5, 5)


class Haha:
    pass
