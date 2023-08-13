use crate::specs::functor::Functor;

pub trait Applicative: Functor {
    fn pure<A>(a: A) -> Self::T<A>;

    fn apply<A, B>(mf: Self::T<fn(A) -> B>, ma: Self::T<A>) -> Self::T<B>;
}
