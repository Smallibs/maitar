use crate::core::hkp::HKP;

pub trait Functor: HKP {
    fn map<A, B>(f: fn(A) -> B, ma: Self::T<A>) -> Self::T<B>;
}
