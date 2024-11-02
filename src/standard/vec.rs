/*
 * MIT License
 *
 * Copyright (c) 2023-2024 Didier Plaindoux
 */

use crate::core::hkp::HKP;
use crate::specs::applicative::Applicative;
use crate::specs::bind::Bind;
use crate::specs::functor::Functor;
use crate::specs::monad::Monad;

pub struct VecK;

impl<'a> HKP<'a> for VecK {
    type T<A: 'a> = Vec<A>;
}

impl<'a> Functor<'a> for VecK {
    fn map<A: 'a, B: 'a, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B + 'a,
    {
        ma.into_iter().map(f).collect()
    }
}

impl<'a> Applicative<'a> for VecK {
    fn pure<A: 'a>(a: A) -> Self::T<A> {
        vec![a]
    }

    fn apply<A: 'a, B: 'a, MAP>(mf: Self::T<MAP>, ma: Self::T<A>) -> Self::T<B>
    where
        A: Clone,
        MAP: Fn(A) -> B + 'a,
    {
        let new_ma = || ma.to_vec().into_iter();
        VecK::join(mf.into_iter().map(|f| new_ma().map(f).collect()).collect())
    }
}

impl<'a> Bind<'a> for VecK {
    fn bind<A: 'a, B: 'a, BIND>(ma: Self::T<A>, mf: BIND) -> Self::T<B>
    where
        BIND: Fn(A) -> Self::T<B> + 'a,
    {
        ma.into_iter().flat_map(mf).collect()
    }
}

impl<'a> Monad<'a> for VecK {}

pub mod infix {
    use crate::core::hkp::HKP;
    use crate::core::transform::Transform;
    use crate::specs::applicative::infix::Applicative;
    use crate::specs::bind::infix::Bind;
    use crate::specs::functor::infix::Functor;
    use crate::specs::monad::infix::Monad;
    use crate::standard::vec::VecK;

    impl<'a, A> HKP<'a> for Vec<A> {
        type T<B: 'a> = Vec<B>;
    }

    impl<'a, A: 'a> Transform<'a, A> for Vec<A> {
        type This = VecK;

        fn hkp_to_self<B: 'a>(a: <Self::This as HKP<'a>>::T<B>) -> Self::T<B> {
            a
        }

        fn self_to_hkp<B: 'a>(a: Self::T<B>) -> <Self::This as HKP<'a>>::T<B> {
            a
        }

        fn to_hkp(self) -> <Self::This as HKP<'a>>::T<A> {
            self
        }
    }

    impl<'a, A: 'a> Functor<'a, A> for Vec<A> {
        type ThisL = VecK;
        type TL<B: 'a> = Vec<B>;
    }

    impl<'a, A: 'a> Applicative<'a, A> for Vec<A> {
        type ThisL = VecK;
        type TL<B: 'a> = Vec<B>;
    }

    impl<'a, A: 'a> Bind<'a, A> for Vec<A> {
        type ThisL = VecK;
        type TL<B: 'a> = Vec<B>;
    }

    impl<'a, A: 'a> Monad<'a, A> for Vec<A> {
        type ThisL = VecK;
        type TL<B: 'a> = Vec<B>;
    }
}
