pub fn test<T: IntoIterator>(val: T) {
    let iter = val.into_iter();
    let (min, max) = iter.size_hint();
    let count = iter.count();
    assert!(count >= min);
    if let Some(max) = max {
        assert!(count <= max);
    }
}

checker!(T, T: IntoIterator);
