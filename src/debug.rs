use core::fmt;

pub fn test<T: fmt::Debug>(val: T) {
    use fmt::Write;

    // Efficient write checker that avoids allocating or doing other work.
    struct WriteCheck(bool);
    impl Write for WriteCheck {
        fn write_char(&mut self, _: char) -> fmt::Result {
            self.0 = true;
            Ok(())
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0 |= !s.is_empty();
            Ok(())
        }
    }
    let mut checker = WriteCheck(false);
    write!(&mut checker, "{:?}", val).expect("debug impl must not return errors");
    assert!(checker.0, "The `Debug` impl is empty");
}

checker!(T, T: fmt::Debug);
