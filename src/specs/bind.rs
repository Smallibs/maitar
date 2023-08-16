use crate::specs::applicative::Applicative;

pub trait Bind: Applicative {
    fn join<A>(mma: Self::T<Self::T<A>>) -> Self::T<A>;

    fn bind<A, B>(ma: Self::T<A>, mf: fn(A) -> Self::T<B>) -> Self::T<B> {
        Self::join(Self::map(mf, ma))
    }
}

pub mod infix {
    use crate::core::hkp::HKP;

    pub trait Bind<A, This: HKP> {
        type T<B>: Bind<B, This>;

        fn this<B>(a: <This as HKP>::T<B>) -> Self::T<B>;

        fn bind<B>(self, mf: fn(A) -> Self::T<B>) -> Self::T<B>;
    }
}
