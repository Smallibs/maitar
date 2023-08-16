use crate::specs::bind::Bind;

pub trait Monad: Bind {
    fn returns<A>(a: A) -> Self::T<A> {
        Self::pure(a)
    }
}
