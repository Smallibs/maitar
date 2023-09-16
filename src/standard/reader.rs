use crate::core::hkp::HKP;
use crate::specs::applicative::Applicative;
use crate::specs::bind::Bind;
use crate::specs::functor::Functor;
use crate::specs::monad::Monad;
use std::marker::PhantomData;

pub struct Reader<'a, E, F: HKP<'a>, A: 'a>(Box<dyn FnOnce(E) -> F::T<A> + 'a>, PhantomData<F>);

pub struct ReaderK<'a, E, F: HKP<'a>>(F, PhantomData<&'a E>);

impl<'a, E, F: HKP<'a>> HKP<'a> for ReaderK<'a, E, F> {
    type T<B: 'a> = Reader<'a, E, F, B>;
}

impl<'e, E, F: Functor<'e> + 'e> Functor<'e> for ReaderK<'e, E, F> {
    fn map<A, B, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B + 'e,
    {
        let Reader(va, _) = ma;
        let run = |e| F::map::<A, B, MAP>(f, va(e));
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
