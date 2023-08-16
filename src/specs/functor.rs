use crate::core::hkp::HKP;

pub trait Functor: HKP {
    fn map<A, B>(f: fn(A) -> B, ma: Self::T<A>) -> Self::T<B>;
}

pub mod infix {
    pub trait Functor<A> {
        type T<B>: Functor<B>;

        fn map<B>(self, f: fn(A) -> B) -> Self::T<B>;
    }
}