use crate::specs::functor::Functor;

pub trait Applicative: Functor {
    fn pure<A>(a: A) -> Self::T<A>;

    fn apply<A, B>(mf: Self::T<fn(A) -> B>, ma: Self::T<A>) -> Self::T<B>;
}

pub mod infix {
    use crate::core::hkp::HKP;
    use crate::specs::applicative::Applicative as Api;

    pub trait Applicative<A> {
        type This: Api;
        type T<B>: Applicative<B>;

        fn from<B>(a: <Self::This as HKP>::T<B>) -> Self::T<B>;

        fn to(self) -> <Self::This as HKP>::T<A>;

        fn apply<B>(self, mf: Self::T<fn(A) -> B>) -> Self::T<B>;
    }
}
