pub fn test<T: Eq>(val: T) {
    if val != val {
        panic!("not reflexive");
    }
}

checker!(T, T: Eq);
