#[macro_export]
macro_rules! and {
    ($expr:expr;) => {
        $expr
    };

    ($first:expr; $($rest:expr;)+) => {
        $first $(&& $rest)+
    };
}
pub use and;

#[macro_export]
macro_rules! or {
    ($expr:expr;) => {
        $expr
    };

    ($first:expr; $($rest:expr;)+) => {
        $first $(|| $rest)+
    };
}
pub use or;

pub trait Kotlin: Sized {
    fn ket<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R
    {
        f(self)
    }

    fn also<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self)
    {
        f(&mut self);
        self
    }
}

impl<T: Sized> Kotlin for T {}
