/*
 * MIT License
 *
 * Copyright (c) 2023-2024 Didier Plaindoux
 */

use crate::core::hkp::HKP;

pub trait Functor<'a>: HKP<'a> {
    fn map<A, B, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B + 'a;
}

pub mod curry {
    use crate::core::functions::curry;
    use crate::specs::functor::Functor as Api;

    pub trait Functor<'a>: Api<'a> {
        fn map<A, B, MAP>(f: MAP) -> Box<dyn FnOnce(Self::T<A>) -> Self::T<B> + 'a>
        where
            Self: 'a,
            MAP: Fn(A) -> B + 'a,
        {
            curry(<Self as Api<'a>>::map)(f)
        }
    }
}

pub mod infix {
    use crate::core::transform::Transform;
    use crate::specs::functor::Functor as Api;

    pub trait Functor<'a, A: 'a>: Transform<'a, A, T<A> = Self::TL<A>, This = Self::ThisL> {
        type ThisL: Api<'a>;
        type TL<B: 'a>: Functor<'a, B>;

        fn map<B, MAP>(self, f: MAP) -> Self::T<B>
        where
            MAP: Fn(A) -> B + 'a,
            Self: Sized,
        {
            Self::hkp_to_self(Self::This::map(f, self.to_hkp()))
        }
    }
}
