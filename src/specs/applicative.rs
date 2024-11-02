/*
 * MIT License
 *
 * Copyright (c) 2023-2024 Didier Plaindoux
 */

use crate::specs::functor::Functor;

pub trait Applicative<'a>: Functor<'a> {
    fn pure<A>(a: A) -> Self::T<A>;

    fn apply<A, B, F>(mf: Self::T<F>, ma: Self::T<A>) -> Self::T<B>
    where
        A: Clone,
        F: Fn(A) -> B + Clone;

    fn lift1<A, B, F>(f: F, ma: Self::T<A>) -> Self::T<B>
    where
        F: Fn(A) -> B + 'a,
    {
        Self::map(f, ma)
    }

    fn lift2<A, B, C, F, G>(f: F, ma: Self::T<A>, mb: Self::T<B>) -> Self::T<C>
    where
        A: Clone,
        B: Clone,
        F: Fn(A) -> G + 'a,
        G: Fn(B) -> C + 'a + Clone,
    {
        Self::apply(Self::map(f, ma), mb)
    }
}

pub mod curry {
    use crate::core::functions::{curry, curry3};
    use crate::specs::applicative::Applicative as Api;

    pub trait Applicative<'a>: Api<'a> {
        fn apply<A, B, F>(mf: Self::T<F>) -> Box<dyn FnOnce(Self::T<A>) -> Self::T<B> + 'a>
        where
            Self: 'a,
            A: Clone,
            F: Fn(A) -> B + Clone,
        {
            curry(<Self as Api<'a>>::apply)(mf)
        }

        fn lift1<A, B, F>(f: F) -> Box<dyn FnOnce(Self::T<A>) -> Self::T<B> + 'a>
        where
            Self: 'a,
            F: Fn(A) -> B + 'a,
        {
            curry(<Self as Api<'a>>::lift1)(f)
        }

        fn lift2<A, B, C, F, G>(
            f: F,
        ) -> Box<dyn FnOnce(Self::T<A>) -> Box<dyn FnOnce(Self::T<B>) -> Self::T<C> + 'a> + 'a>
        where
            Self: 'a,
            A: Clone,
            B: Clone,
            F: Fn(A) -> G + Clone + 'a,
            G: Fn(B) -> C + Clone + 'a,
        {
            curry3(<Self as Api<'a>>::lift2)(f)
        }
    }
}

pub mod infix {
    use crate::core::transform::Transform;
    use crate::specs::applicative::Applicative as Api;

    pub trait Applicative<'a, A: 'a>:
        Transform<'a, A, T<A> = Self::TL<A>, This = Self::ThisL>
    {
        type ThisL: Api<'a>;
        type TL<B: 'a>: Applicative<'a, B>;

        fn pure<B>(a: B) -> Self::T<B> {
            Self::hkp_to_self(Self::This::pure(a))
        }

        fn apply<B, MAP>(self, mf: Self::T<MAP>) -> Self::T<B>
        where
            A: Clone,
            MAP: Fn(A) -> B + Clone,
            Self: Sized,
        {
            Self::hkp_to_self(Self::This::apply(
                Self::self_to_hkp::<MAP>(mf),
                self.to_hkp(),
            ))
        }
    }
}
