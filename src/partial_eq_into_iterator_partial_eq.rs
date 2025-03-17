pub fn test<T: IntoIterator + PartialEq>((val1, val2): (T, T)) where T::Item: PartialEq {
    if val1 == val2 {
        assert!(val1.into_iter().zip(val2).all(|(a, b)| a == b));
    } else {
        assert!(val1.into_iter().zip(val2).any(|(a, b)| a != b));
    }
}

checker!((T, T), T: IntoIterator + PartialEq, T::Item: PartialEq);
