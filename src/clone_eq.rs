pub fn test<T: Clone + Eq>(val1: T) {
    let val2 = val1.clone();
    if val1 != val2 {
        panic!("cloned values are not equal");
    }
}

checker!(T, T: Clone + Eq);
