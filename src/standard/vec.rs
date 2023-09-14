use crate::core::hkp::HKP;
use crate::specs::applicative::Applicative;
use crate::specs::bind::Bind;
use crate::specs::functor::Functor;
use crate::specs::monad::Monad;

pub struct VecK;

impl HKP for VecK {
    type T<A> = Vec<A>;
}

impl Functor for VecK {
    fn map<A, B, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
        where
            MAP: Fn(A) -> B,
    {
        ma.into_iter().map(f).collect()
    }
}

impl Applicative for VecK {
    fn pure<A>(a: A) -> Self::T<A> {
        vec![a]
    }

    fn apply<A, B, MAP>(mf: Self::T<MAP>, ma: Self::T<A>) -> Self::T<B>
        where
            A: Clone,
            MAP: Fn(A) -> B,
    {
        let new_ma = || ma.to_vec().into_iter();
        VecK::join(mf.into_iter().map(|f| new_ma().map(f).collect()).collect())
    }
}

impl Bind for VecK {
    fn join<A>(mma: Self::T<Self::T<A>>) -> Self::T<A> {
        mma.into_iter().flatten().collect()
    }

    fn bind<A, B, BIND>(ma: Self::T<A>, mf: BIND) -> Self::T<B>
        where
            BIND: Fn(A) -> Self::T<B>,
    {
        ma.into_iter().flat_map(mf).collect()
    }
}

impl Monad for VecK {}

pub mod infix {
    use crate::core::hkp::HKP;
    use crate::core::transform::Transform;
    use crate::specs::applicative::infix::Applicative;
    use crate::specs::bind::infix::Bind;
    use crate::specs::functor::infix::Functor;
    use crate::specs::monad::infix::Monad;
    use crate::standard::vec::VecK;

    impl<A> HKP for Vec<A> {
        type T<B> = Vec<B>;
    }

    impl<A> Transform<A> for Vec<A> {
        type This = VecK;

        fn from_hkp<B>(a: <Self::This as HKP>::T<B>) -> Self::T<B> {
            a
        }

        fn from_self<B>(a: Self::T<B>) -> <Self::This as HKP>::T<B> {
            a
        }

        fn to_hkp(self) -> <Self::This as HKP>::T<A> {
            self
        }
    }

    impl<A> Functor<A> for Vec<A> {
        type ThisL = VecK;
        type TL<B> = Vec<B>;
    }

    impl<A> Applicative<A> for Vec<A> {
        type ThisL = VecK;
        type TL<B> = Vec<B>;
    }

    impl<A> Bind<A> for Vec<A> {
        type ThisL = VecK;
        type TL<B> = Vec<B>;
    }

    impl<A> Monad<A> for Vec<A> {
        type ThisL = VecK;
        type TL<B> = Vec<B>;
    }
}
