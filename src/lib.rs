//! # Standard Rust test suite
//! 
//! While not always explicitly documented, it is generally expected that the implementation of
//! traits from Rust's `std` has certain properties that cannot be expressed in the type system
//! alone. For instance, most people assume that if a type `T` implements both `Clone` and `Eq`
//! then `x == x.clone()` holds for all values `x` of `T`. Thus it's often desirable for the impls
//! to have such properties. In some cases (such as `Eq`) the properties are explicitly documented
//! as required.
//! 
//! In any case, the only way to enforce correctness is to test the impls but that can be tedious
//! and repetitive. This crate aims to help with that. It provides a set of standard tests that can
//! be invoked easily.
//! 
//! The recommended way is to implement [`Arbitrary`](arbitrary::Arbitrary) from crate [`arbitrary`]
//! for your types and then invoke `standard_checks!(YourTypeHere)` macro to magically implement a
//! test for your type. You can then use the `one_iteration` method within your fuzz loop.
//! 
//! The macro uses hacky "specialization" to run appropriate tests for your type based on which
//! traits it implements.
//! 
//! Alternatively, if you for some reason cannot implement `Arbitrary` but can generate values
//! differently, you can simply manually call approriate functions in the crate that provide the
//! checks. For instance you could use [`kani`] to model-check that your implementation is correct.
//! 
//! [`kani`]: https://github.com/model-checking/kani

#[cfg(feature = "arbitrary")]
pub extern crate arbitrary;

macro_rules! checker {
    ($arg:ty, $($bounds:tt)*) => {
        #[doc(hidden)]
        #[derive(Default)]
        pub struct Checker<T>(core::marker::PhantomData<fn(T)>);

        impl<T> crate::trait_hack::CheckerArg for Checker<T> {
            type Arg = $arg;
        }

        impl<T> crate::trait_hack::Checker for Checker<T> where $($bounds)* {
            fn run_check(arg: Self::Arg) {
                test(arg)
            }
        }
    }
}

/// Checks that cloned iterator yields the same values.
pub mod clone_iterator_of_eq;
/// Checks combination of `Clone + Eq`.
pub mod clone_eq;
/// Checks reflexivity.
pub mod eq;
/// Checks that for equal values equal bytes are given to a hasher.
pub mod eq_hash;
/// Checks that the length of `ExactSizeIterator` matches `size_hint`
pub mod exact_size_iterator;
/// Checks that the iterator continues returning `None` once it reached the end.
pub mod fused_iterator;
/// Checks that `size_hint` behaves correctly.
pub mod iterator;
/// Checks commutativity and transitivity.
pub mod partial_eq;
/// Checks combination of `IntoIterator + PartialEq` where the items implement `PartialEq`.
pub mod partial_eq_into_iterator_partial_eq;

/// A trait implemented by types that have their traits auto-checked.
pub trait StandardChecks {
    /// Performs one test iteration.
    ///
    /// The bytes produced by a fuzzer need to be passed as `dna`. To properly check your type you
    /// need to run this many times using different inputs.
    fn one_iteration(dna: &[u8]);
}

#[cfg(feature = "arbitrary")]
#[doc(hidden)]
#[macro_export]
macro_rules! __do_checks {
    ($dna:expr, $type:ty, $($module:ident),+ $(,)?) => {
        {
            use $crate::trait_hack::RunCheck;

            $(
                let mut unstructured = $crate::arbitrary::Unstructured::new($dna);
                if let Ok(value) = unstructured.arbitrary::<<$crate::$module::Checker::<$type> as $crate::trait_hack::CheckerArg>::Arg>() {
                    0.run_check(value, $crate::$module::Checker::<$type>::default());
                }
            )+
        }
    }
}

/// Automagically implements `StandardChecks` for your type given as paramter.
///
/// Note that this cannot support generics, however you can just instantiate your type with
/// appropriate type parameters.
#[cfg(feature = "arbitrary")]
#[macro_export]
macro_rules! standard_checks {
    ($type:ty) => {
        impl $crate::StandardChecks for $type {
            fn one_iteration(dna: &[u8]) {
                $crate::__do_checks! {
                    dna,
                    $type,
                    clone_iterator_of_eq,
                    clone_eq,
                    eq,
                    eq_hash,
                    exact_size_iterator,
                    fused_iterator,
                    iterator,
                    partial_eq,
                    partial_eq_into_iterator_partial_eq,
                }
            }
        }
    };
}

#[doc(hidden)]
pub mod trait_hack;

#[cfg(feature = "arbitrary")]
#[cfg(test)]
mod tests {
    use super::StandardChecks;

    super::standard_checks!(bool);
    super::standard_checks!([bool; 1]);

    #[test]
    fn core_bool() {
        bool::one_iteration(&[0, 0, 0]);
        bool::one_iteration(&[0, 0, 1]);
        bool::one_iteration(&[0, 1, 0]);
        bool::one_iteration(&[0, 1, 1]);
        bool::one_iteration(&[1, 0, 0]);
        bool::one_iteration(&[1, 0, 1]);
        bool::one_iteration(&[1, 1, 0]);
        bool::one_iteration(&[1, 1, 1]);
    }

    #[test]
    fn core_array_of_bools() {
        <[bool; 1]>::one_iteration(&[0, 0, 0]);
        <[bool; 1]>::one_iteration(&[0, 0, 1]);
        <[bool; 1]>::one_iteration(&[0, 1, 0]);
        <[bool; 1]>::one_iteration(&[0, 1, 1]);
        <[bool; 1]>::one_iteration(&[1, 0, 0]);
        <[bool; 1]>::one_iteration(&[1, 0, 1]);
        <[bool; 1]>::one_iteration(&[1, 1, 0]);
        <[bool; 1]>::one_iteration(&[1, 1, 1]);
    }
}
