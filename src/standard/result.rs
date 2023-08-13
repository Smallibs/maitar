use crate::specs::applicative::Applicative;
use crate::specs::functor::Functor;
use crate::specs::hkp::HKP;
use crate::specs::monad::Monad;
use std::marker::PhantomData;

pub struct ResultK<E> {
    _ignore: PhantomData<E>,
}

impl<E> HKP for ResultK<E> {
    type T<A> = Result<A, E>;
}

impl<E> Functor for ResultK<E> {
    fn map<A, B>(f: fn(A) -> B, ma: Self::T<A>) -> Self::T<B> {
        match ma {
            Ok(a) => Ok(f(a)),
            Err(e) => Err(e),
        }
    }
}

impl<E> Applicative for ResultK<E> {
    fn pure<A>(a: A) -> Self::T<A> {
        Ok(a)
    }

    fn apply<A, B>(mf: Self::T<fn(A) -> B>, ma: Self::T<A>) -> Self::T<B> {
        match mf {
            Ok(f) => ResultK::map(f, ma),
            Err(e) => Err(e),
        }
    }
}

impl<E> Monad for ResultK<E> {
    fn join<A>(mma: Self::T<Self::T<A>>) -> Self::T<A> {
        match mma {
            Ok(ma) => ma,
            Err(e) => Err(e),
        }
    }
}
