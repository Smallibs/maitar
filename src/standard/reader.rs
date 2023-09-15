use crate::core::hkp::HKP;
use crate::specs::functor::Functor;
use std::marker::PhantomData;

pub struct Reader<'a, E, F: HKP<'a>, A: 'a>(F, Box<dyn FnOnce(E) -> F::T<A> + 'a>);

pub struct ReaderK<'a, E, F: HKP<'a>>(PhantomData<&'a E>, PhantomData<F>);

impl<'a, E, F: HKP<'a>> HKP<'a> for ReaderK<'a, E, F> {
    type T<B: 'a> = Reader<'a, E, F, B>;
}

impl<'e, E, F: Functor<'e> + 'e> Functor<'e> for ReaderK<'e, E, F> {
    fn map<A, B, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B + 'e,
    {
        let Reader(functor, va) = ma;
        let run = |e| F::map::<A, B, MAP>(f, va(e));
        Reader(functor, Box::new(run))
    }
}
