use core::hash::Hash;

pub fn test<T: Eq + Hash>((val1, val2): (T, T)) {
    struct DummyHasher(Vec<u8>);

    impl core::hash::Hasher for DummyHasher {
        fn finish(&self) -> u64 {
            panic!("Hash implementation shouldn't call finish() on Hasher");
        }

        fn write(&mut self, bytes: &[u8]) {
            self.0.extend_from_slice(bytes);
        }
    }

    if val1 == val2 {
        let mut hasher1 = DummyHasher(Default::default());
        val1.hash(&mut hasher1);
        let mut hasher2 = DummyHasher(Default::default());
        val2.hash(&mut hasher2);
        assert_eq!(hasher1.0, hasher2.0);
    }
}

checker!((T, T), T: Eq + Hash);
