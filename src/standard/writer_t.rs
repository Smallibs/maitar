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
use crate::specs::monoid::Monoid;
use std::marker::PhantomData;

pub struct WriterT<'a, E: 'a, F: HKP<'a>, A: 'a>(F::T<(E, A)>);

pub struct WriterK<'a, E, F: HKP<'a>>(PhantomData<&'a E>, PhantomData<F>);

impl<'a, E, F: HKP<'a>> HKP<'a> for WriterK<'a, E, F> {
    type T<B: 'a> = WriterT<'a, E, F, B>;
}

impl<'a, E, F: Functor<'a>> Functor<'a> for WriterK<'a, E, F> {
    fn map<A, B, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B + 'a,
    {
        let WriterT(va) = ma;
        WriterT(F::map(move |(e, a)| (e, f(a)), va))
    }
}

impl<'a, E: Monoid<T = E> + Copy, F: Applicative<'a> + 'a> Applicative<'a> for WriterK<'a, E, F> {
    fn pure<A>(a: A) -> Self::T<A> {
        WriterT(F::pure((E::neutral(), a)))
    }

    fn apply<A, B, M>(mf: Self::T<M>, ma: Self::T<A>) -> Self::T<B>
    where
        A: Clone,
        M: Fn(A) -> B + 'a + Clone,
    {
        WriterT(F::lift2(
            |(t, f)| move |(u, g)| (E::compose(t, u), f(g)),
            mf.0,
            ma.0,
        ))
    }
}

impl<'a, E: Monoid<T = E> + Copy, F: Bind<'a> + 'a> Bind<'a> for WriterK<'a, E, F> {
    fn bind<A, B, BIND>(ma: Self::T<A>, mf: BIND) -> Self::T<B>
    where
        BIND: Fn(A) -> Self::T<B> + 'a,
    {
        WriterT(F::bind(ma.0, move |(x, t)| {
            F::map(move |(y, u)| (E::compose(x, y), u), mf(t).0)
        }))
    }
}

impl<'a, E: Monoid<T = E> + Copy, F: Monad<'a> + 'a> Monad<'a> for WriterK<'a, E, F> {}
