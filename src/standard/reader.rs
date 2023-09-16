use crate::core::hkp::HKP;
use crate::specs::applicative::Applicative;
use crate::specs::bind::Bind;
use crate::specs::functor::Functor;
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

impl<'e, E: Clone, F: Applicative<'e> + 'e> Applicative<'e> for ReaderK<'e, E, F> {
    fn pure<A: 'e>(a: A) -> Self::T<A> {
        Reader(Box::new(|_| F::pure(a)), PhantomData)
    }

    fn apply<A, B, MAP>(mf: Self::T<MAP>, ma: Self::T<A>) -> Self::T<B>
    where
        A: Clone,
        MAP: Fn(A) -> B,
    {
        let Reader(vf, _) = mf;
        let Reader(va, _) = ma;
        let run = |e: E| F::apply(vf(e.clone()), va(e));
        Reader(Box::new(run), PhantomData)
    }
}
