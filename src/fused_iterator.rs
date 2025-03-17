pub fn test<T: IntoIterator>(val: T) where T::IntoIter: core::iter::FusedIterator {
    let mut iter = val.into_iter();
    for _ in &mut iter {}
    assert!(iter.next().is_none());
    assert!(iter.next().is_none());
    assert!(iter.next().is_none());
    assert!(iter.next().is_none());
}

checker!(T, T: IntoIterator, T::IntoIter: core::iter::FusedIterator);
