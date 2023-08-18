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
            MAP: Fn(A) -> B,
    {
        todo!()
        // VecK::join(mf.into_iter().map(|f| VecK::map(f, ma)).collect())
    }
}

impl Bind for VecK {
    fn join<A>(mma: Self::T<Self::T<A>>) -> Self::T<A> {
        mma.into_iter().flatten().collect()
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

    impl<'a, A> Transform<A> for Vec<A> {
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

    impl<'a, A> Functor<A> for Vec<A> {
        type ThisL = VecK;
        type TL<B> = Vec<B>;
    }

    impl<'a, A> Applicative<A> for Vec<A> {
        type ThisL = VecK;
        type TL<B> = Vec<B>;
    }

    impl<'a, A> Bind<A> for Vec<A> {
        type ThisL = VecK;
        type TL<B> = Vec<B>;
    }

    impl<'a, A> Monad<A> for Vec<A> {
        type ThisL = VecK;
        type TL<B> = Vec<B>;
    }
}
