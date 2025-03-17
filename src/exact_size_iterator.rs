pub fn test<T: IntoIterator>(val: T) where T::IntoIter: ExactSizeIterator {
    let iter = val.into_iter();
    let size_hint = iter.size_hint();
    let len = iter.len();
    assert_eq!(size_hint, (len, Some(len)));
}

checker!(T, T: IntoIterator, T::IntoIter: ExactSizeIterator);
