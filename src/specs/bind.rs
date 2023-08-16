use crate::specs::applicative::Applicative;

pub trait Bind: Applicative {
    fn join<A>(mma: Self::T<Self::T<A>>) -> Self::T<A>;

    fn bind<A, B>(ma: Self::T<A>, mf: fn(A) -> Self::T<B>) -> Self::T<B> {
        Self::join(Self::map(mf, ma))
    }
}

pub mod infix {
    use crate::core::hkp::HKP;
    use crate::specs::bind::Bind as Api;

    pub trait Bind<A> {
        type This: Api;
        type T<B>: Bind<B>;

        fn from<B>(a: <Self::This as HKP>::T<B>) -> Self::T<B>;

        fn to(self) -> <Self::This as HKP>::T<A>;

        fn bind<B>(self, mf: fn(A) -> Self::T<B>) -> Self::T<B>;
    }
}
