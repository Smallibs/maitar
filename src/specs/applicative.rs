use crate::specs::functor::Functor;

pub trait Applicative: Functor {
    fn pure<A>(a: A) -> Self::T<A>;

    fn apply<A, B>(mf: Self::T<fn(A) -> B>, ma: Self::T<A>) -> Self::T<B>;
}

pub mod infix {
    use crate::core::hkp::HKP;

    pub trait Applicative<A, This: HKP> {
        type T<B>: Applicative<B, This>;

        fn from<B>(a: <This as HKP>::T<B>) -> Self::T<B>;

        fn to(self) -> <This as HKP>::T<A>;

        fn apply<B>(self, mf: Self::T<fn(A) -> B>) -> Self::T<B>;
    }
}
