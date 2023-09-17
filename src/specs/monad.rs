/*
 * MIT License
 *
 * Copyright (c) 2023 Didier Plaindoux
 */

use crate::specs::bind::Bind;

pub trait Monad<'a>: Bind<'a> {
    fn returns<A>(a: A) -> Self::T<A> {
        Self::pure(a)
    }
}

pub mod infix {
    use crate::core::transform::Transform;
    use crate::specs::bind::Bind;
    use crate::specs::monad::Monad as Api;

    pub trait Monad<'a, A: 'a>: Transform<'a, A, T<A> = Self::TL<A>, This = Self::ThisL> {
        type ThisL: Api<'a>;

        type TL<B: 'a>: Monad<'a, B>;

        fn returns<B>(b: B) -> Self::T<B> {
            Self::hkp_to_self(Self::This::returns(b))
        }

        fn join<B>(mma: Self::T<Self::T<B>>) -> Self::T<B> {
            Self::hkp_to_self(<Self::This as Bind>::bind(Self::self_to_hkp(mma), |a| {
                Self::self_to_hkp::<B>(a)
            }))
        }

        fn bind<B, BIND>(self, mf: BIND) -> Self::T<B>
        where
            BIND: Fn(A) -> Self::T<B> + 'a,
            Self: Sized,
        {
            Self::hkp_to_self(<Self::This as Bind>::bind(self.to_hkp(), move |a| {
                Self::self_to_hkp::<B>(mf(a))
            }))
        }
    }
}
