pub fn test<T: IntoIterator>(val: T) where T::Item: Eq, T::IntoIter: Clone {
    let val1 = val.into_iter();
    let val2 = val1.clone();
    assert!(val1.zip(val2).all(|(a, b)| a == b));
}

checker!(T, T: IntoIterator, T::Item: Eq, T::IntoIter: Clone);
