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
use std::marker::PhantomData;

pub struct Reader<'a, E, F: HKP<'a>, A: 'a>(
    Box<dyn FnOnce(E) -> <F as HKP<'a>>::T<A> + 'a>,
    PhantomData<F>,
);

pub struct ReaderK<'a, E, F: HKP<'a>>(PhantomData<&'a E>, PhantomData<F>);

impl<'a, E, F: HKP<'a>> HKP<'a> for ReaderK<'a, E, F> {
    type T<B: 'a> = Reader<'a, E, F, B>;
}

impl<'e, E, F: Functor<'e> + 'e> Functor<'e> for ReaderK<'e, E, F> {
    fn map<A, B, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B + 'e,
    {
        let run = |e| F::map::<A, B, MAP>(f, ma.0(e));
        Reader(Box::new(run), PhantomData)
    }
}

impl<'e, E: Copy, F: Applicative<'e> + 'e> Applicative<'e> for ReaderK<'e, E, F> {
    fn pure<A: 'e>(a: A) -> Self::T<A> {
        let run = |_| F::pure(a);
        Reader(Box::new(run), PhantomData)
    }

    fn apply<A, B, MAP>(mf: Self::T<MAP>, ma: Self::T<A>) -> Self::T<B>
    where
        A: Clone,
        MAP: Fn(A) -> B,
    {
        let Reader(vf, _) = mf;
        let Reader(va, _) = ma;
        let run = |e: E| F::apply(vf(e), va(e));
        Reader(Box::new(run), PhantomData)
    }
}

impl<'e, E: Copy, F: Bind<'e> + 'e> Bind<'e> for ReaderK<'e, E, F> {
    fn bind<A, B, BIND>(ma: Self::T<A>, f: BIND) -> Self::T<B>
    where
        BIND: Fn(A) -> Self::T<B> + 'e,
    {
        let Reader(va, _) = ma;
        let run = |e1: E| {
            F::bind(va(e1), move |e2: A| {
                let Reader(v, _) = f(e2);
                v(e1)
            })
        };

        Reader(Box::new(run), PhantomData)
    }
}

impl<'e, E: Copy, F: Bind<'e> + 'e> Monad<'e> for ReaderK<'e, E, F> {}

impl<'e, E: Copy + 'e, F: Monad<'e> + 'e> ReaderK<'e, E, F> {
    pub fn reader<A>(f: fn(E) -> F::T<A>) -> Reader<'e, E, F, A> {
        Reader(Box::new(f), PhantomData)
    }

    pub fn run<A>(reader: Reader<'e, E, F, A>) -> Box<dyn FnOnce(E) -> F::T<A> + 'e> {
        let Reader(f, _) = reader;
        f
    }

    pub fn ask() -> Reader<'e, E, F, E> {
        Reader(Box::new(F::returns), PhantomData)
    }

    pub fn local<A>(f: Box<dyn Fn(E) -> E>, reader: Reader<'e, E, F, A>) -> Reader<'e, E, F, A> {
        let Reader(run, _) = reader;
        Reader(Box::new(move |e| run(f(e))), PhantomData)
    }
}

pub mod infix {
    use crate::core::hkp::HKP;
    use crate::core::transform::Transform;
    use crate::specs::applicative::infix::Applicative;
    use crate::specs::bind::infix::Bind;
    use crate::specs::functor::infix::Functor;
    use crate::specs::monad::infix::Monad;
    use crate::standard::reader::{Reader, ReaderK};

    impl<'a, E, F: HKP<'a>, A: 'a> HKP<'a> for Reader<'a, E, F, A> {
        type T<B: 'a> = Reader<'a, E, F, B>;
    }

    impl<'a, E: 'a, F: HKP<'a>, A: 'a> Transform<'a, A> for Reader<'a, E, F, A> {
        type This = ReaderK<'a, E, F>;

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

    impl<'a, E: 'a, F: HKP<'a> + crate::specs::functor::Functor<'a> + 'a, A: 'a> Functor<'a, A>
        for Reader<'a, E, F, A>
    {
        type ThisL = ReaderK<'a, E, F>;
        type TL<B: 'a> = Reader<'a, E, F, B>;
    }

    impl<'a, E: 'a + Copy, F: HKP<'a> + crate::specs::applicative::Applicative<'a> + 'a, A: 'a>
        Applicative<'a, A> for Reader<'a, E, F, A>
    {
        type ThisL = ReaderK<'a, E, F>;
        type TL<B: 'a> = Reader<'a, E, F, B>;
    }

    impl<'a, E: 'a + Copy, F: HKP<'a> + crate::specs::bind::Bind<'a> + 'a, A: 'a> Bind<'a, A>
        for Reader<'a, E, F, A>
    {
        type ThisL = ReaderK<'a, E, F>;
        type TL<B: 'a> = Reader<'a, E, F, B>;
    }

    impl<'a, E: 'a + Copy, F: HKP<'a> + crate::specs::monad::Monad<'a> + 'a, A: 'a> Monad<'a, A>
        for Reader<'a, E, F, A>
    {
        type ThisL = ReaderK<'a, E, F>;
        type TL<B: 'a> = Reader<'a, E, F, B>;
    }
}
