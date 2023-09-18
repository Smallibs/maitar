/*
 * MIT License
 *
 * Copyright (c) 2023 Didier Plaindoux
 */

use crate::core::types::Fun;
use crate::specs::functor::Functor;

pub trait Applicative<'a>: Functor<'a> {
    fn pure<A>(a: A) -> Self::T<A>;

    fn apply<A, B, MAP>(mf: Self::T<MAP>, ma: Self::T<A>) -> Self::T<B>
    where
        A: Clone,
        MAP: Fn(A) -> B;

    fn lift1<A, B>(f: Fun<A, B>, ma: Self::T<A>) -> Self::T<B> {
        Self::map(f, ma)
    }

    fn lift2<A: Clone, B: Clone, C>(
        f: Fun<A, Fun<B, C>>,
        ma: Self::T<A>,
        mb: Self::T<B>,
    ) -> Self::T<C> {
        Self::apply(Self::apply(Self::pure(f), ma), mb)
    }
}

pub mod curry {
    use crate::core::types::{Fun, FunOnceLT};
    use crate::specs::applicative::Applicative as Api;

    pub trait Applicative<'a>: Api<'a> {
        fn apply<A, B, MAP>(mf: Self::T<MAP>) -> FunOnceLT<'a, Self::T<A>, Self::T<B>>
        where
            Self: 'a,
            A: Clone,
            MAP: Fn(A) -> B,
        {
            Box::new(move |ma| <Self as Api<'a>>::apply(mf, ma))
        }

        fn lift1<A, B>(f: Fun<A, B>) -> FunOnceLT<'a, Self::T<A>, Self::T<B>> {
            Box::new(move |ma| <Self as Api<'a>>::lift1(f, ma))
        }

        fn lift2<A, B, C>(
            f: Fun<A, Fun<B, C>>,
        ) -> FunOnceLT<'a, Self::T<A>, FunOnceLT<'a, Self::T<B>, Self::T<C>>>
        where
            Self: 'a,
            A: Clone,
            B: Clone,
        {
            Box::new(move |ma| Box::new(move |mb| <Self as Api<'a>>::lift2(f, ma, mb)))
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
            MAP: Fn(A) -> B,
            Self: Sized,
        {
            Self::hkp_to_self(Self::This::apply(
                Self::self_to_hkp::<MAP>(mf),
                self.to_hkp(),
            ))
        }
    }
}
