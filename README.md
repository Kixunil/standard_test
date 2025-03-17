# Standard Rust test suite

While not always explicitly documented, it is generally expected that the implementation of traits from Rust's `std` has certain properties that cannot be expressed in the type system alone. For instance, most people assume that if a type `T` implements both `Clone` and `Eq` then `x == x.clone()` holds for all values `x` of `T`. Thus it's often desirable for the impls to have such properties. In some cases (such as `Eq`) the properties are explicitly documented as required.

In any case, the only way to enforce correctness is to test the impls but that can be tedious and repetitive. This crate aims to help with that. It provides a set of standard tests that can be invoked easily.

The recommended way is to implement `Arbitrary` from crate [`arbitrary`] for your types and then invoke `standard_checks!(YourTypeHere)` macro to magically implement a test for your type. You can then use the `one_iteration` method within your fuzz loop.

The macro uses hacky "specialization" to run appropriate tests for your type based on which traits it implements.

Alternatively, if you for some reason cannot implement `Arbitrary` but can generate values differently, you can simply manually call approriate functions in the crate that provide the checks. For instance you could use [`kani`] to model-check that your implementation is correct.

[`arbitrary`]: https://docs.rs/arbitrary
[`kani`]: https://github.com/model-checking/kani
