## cortex

Neural network components library.

### Cube

Tensor library.

Features
+ Type parametrization for tensor datatype and shape.
TODO think about limited dependent type checking on shape


```rust
Tensor<b8, A, B, C> 
```


+ Operators as traits for composition, categorized into
  + new             (...) -> Ten            Constructors
  + mold            Ten -> Ten              Geometric Morphisms (operates purely on "extrinsic container") TODO make rigorous
  + apply           Ten -> Ten              Applicative Morphisms (Maps from type to type intrinsically as well as extrinsically)
  + reduce          Ten > Scalar            Reductions (Maps from type to reduced axis type or a scalar) TODO clarify / separate
  + split           Ten -> (Ten, Ten)       One to many
  + merge           (Ten, Ten) -> Ten       Many to one

TODO example

+ Expression equivalences.

TODO example


### Block

Neural network components library. Similar to `torch.module`.

### Graph

TODO: Extend utiities to graph neural networks.
