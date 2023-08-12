use crate::specs::applicative::Applicative;

pub trait Monad: Applicative {
    fn returns<A>(a: A) -> Self::T<A> {
        Self::pure(a)
    }

    fn join<A>(mma: Self::T<Self::T<A>>) -> Self::T<A>;

    fn bind<A, B>(mf: fn(A) -> Self::T<B>, ma: Self::T<A>) -> Self::T<B> {
        Self::join(Self::map(mf, ma))
    }
}
