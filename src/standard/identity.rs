/*
 * MIT License
 *
 * Copyright (c) 2023 Didier Plaindoux
 */

use crate::core::hkp::HKP;
use crate::specs::applicative::Applicative;
use crate::specs::bind::Bind;
use crate::specs::functor::Functor;
use crate::specs::monad::Monad;

pub struct IdentityK;

pub struct Identity<A> {
    value: A,
}

impl<'a> HKP<'a> for IdentityK {
    type T<A: 'a> = Identity<A>;
}

impl<'a> Functor<'a> for IdentityK {
    fn map<A: 'a, B: 'a, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B + 'a,
    {
        Identity { value: f(ma.value) }
    }
}

impl<'a> Applicative<'a> for IdentityK {
    fn pure<A: 'a>(a: A) -> Self::T<A> {
        Identity { value: a }
    }

    fn apply<A: 'a, B: 'a, MAP>(mf: Self::T<MAP>, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B + 'a,
    {
        Self::map(mf.value, ma)
    }
}

impl<'a> Bind<'a> for IdentityK {
    fn bind<A: 'a, B: 'a, BIND>(ma: Self::T<A>, f: BIND) -> Self::T<B>
    where
        BIND: Fn(A) -> Self::T<B> + 'a,
    {
        f(ma.value)
    }
}

impl Monad<'_> for IdentityK {}

pub mod infix {
    use crate::core::hkp::HKP;
    use crate::core::transform::Transform;
    use crate::specs::applicative::infix::Applicative;
    use crate::specs::bind::infix::Bind;
    use crate::specs::functor::infix::Functor;
    use crate::specs::monad::infix::Monad;
    use crate::standard::identity::{Identity, IdentityK};

    impl<'a, A> HKP<'a> for Identity<A> {
        type T<B: 'a> = Identity<B>;
    }

    impl<'a, A: 'a> Transform<'a, A> for Identity<A> {
        type This = IdentityK;

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

    impl<'a, A: 'a> Functor<'a, A> for Identity<A> {
        type ThisL = IdentityK;
        type TL<B: 'a> = Identity<B>;
    }

    impl<'a, A: 'a> Applicative<'a, A> for Identity<A> {
        type ThisL = IdentityK;
        type TL<B: 'a> = Identity<B>;
    }

    impl<'a, A: 'a> Bind<'a, A> for Identity<A> {
        type ThisL = IdentityK;
        type TL<B: 'a> = Identity<B>;
    }

    impl<'a, A: 'a> Monad<'a, A> for Identity<A> {
        type ThisL = IdentityK;
        type TL<B: 'a> = Identity<B>;
    }
}
