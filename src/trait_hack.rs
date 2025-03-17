pub trait RunCheck<T, C> {
    fn run_check(self, value: T, check: C);
}

impl<T, C> RunCheck<T, C> for i32 where C: Checker<Arg=T> {
    fn run_check(self, value: T, _check: C) {
        C::run_check(value);
    }
}

impl<T, C> RunCheck<T, C> for u32 {
    fn run_check(self, _value: T, _check: C) { }
}

pub trait CheckerArg {
    type Arg;
}

pub trait Checker: CheckerArg {
    fn run_check(arg: Self::Arg);
}
