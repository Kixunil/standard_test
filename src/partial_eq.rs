pub fn test<T: PartialEq>((val1, val2, val3): (T, T, T)) {
    if val1 == val2 {
        if val1 != val1 {
            panic!("not commutative");
        }
        if val2 == val3 && val1 != val3 {
            panic!("not transitive");
        }
    }
}

checker!((T, T, T), T: PartialEq);
