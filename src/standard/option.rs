use crate::core::hkp::HKP;
use crate::specs::applicative::Applicative;
use crate::specs::bind::Bind;
use crate::specs::functor::Functor;
use crate::specs::monad::Monad;

pub struct OptionK;

impl HKP for OptionK {
    type T<A> = Option<A>;
}

impl Functor for OptionK {
    fn map<A, B>(f: fn(A) -> B, ma: Self::T<A>) -> Self::T<B> {
        ma.map(f)
    }
}

impl Applicative for OptionK {
    fn pure<A>(a: A) -> Self::T<A> {
        Some(a)
    }

    fn apply<A, B>(mf: Self::T<fn(A) -> B>, ma: Self::T<A>) -> Self::T<B> {
        match mf {
            Some(f) => Self::map(f, ma),
            None => None,
        }
    }
}

impl Bind for OptionK {
    fn join<A>(mma: Self::T<Self::T<A>>) -> Self::T<A> {
        match mma {
            Some(ma) => ma,
            None => None,
        }
    }
}

impl Monad for OptionK {}
