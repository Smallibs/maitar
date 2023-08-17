use crate::core::hkp::HKP;

pub trait Transform<A> {
    type This: HKP;
    type T<B>;

    fn from_hkp<B>(a: <Self::This as HKP>::T<B>) -> Self::T<B>;

    fn from_self<B>(a: Self::T<B>) -> <Self::This as HKP>::T<B>;

    fn to_hkp(self) -> <Self::This as HKP>::T<A>;
}
