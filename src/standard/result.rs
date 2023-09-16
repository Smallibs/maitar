use std::marker::PhantomData;

use crate::core::hkp::HKP;
use crate::specs::applicative::Applicative;
use crate::specs::bind::Bind;
use crate::specs::functor::Functor;
use crate::specs::monad::Monad;

pub struct ResultK<E> {
    _ignore: PhantomData<E>,
}

impl<'a, E> HKP<'a> for ResultK<E> {
    type T<A: 'a> = Result<A, E>;
}

impl<'a, E> Functor<'a> for ResultK<E> {
    fn map<A: 'a, B: 'a, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B + 'a,
    {
        ma.map(f)
    }
}

impl<'a, E> Applicative<'a> for ResultK<E> {
    fn pure<A: 'a>(a: A) -> Self::T<A> {
        Ok(a)
    }

    fn apply<A: 'a, B: 'a, MAP>(mf: Self::T<MAP>, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B + 'a,
    {
        match mf {
            Ok(f) => Self::map(f, ma),
            Err(e) => Err(e),
        }
    }
}

impl<'a, E: 'a> Bind<'a> for ResultK<E> {
    fn bind<A: 'a, B: 'a, BIND>(ma: Self::T<A>, f: BIND) -> Self::T<B>
    where
        BIND: Fn(A) -> Self::T<B> + 'a,
    {
        match ma {
            Ok(a) => f(a),
            Err(e) => Err(e),
        }
    }
}

impl<'a, E: 'a> Monad<'a> for ResultK<E> {}

mod infix {
    use crate::core::hkp::HKP;
    use crate::core::transform::Transform;
    use crate::specs::applicative::infix::Applicative;
    use crate::specs::bind::infix::Bind;
    use crate::specs::functor::infix::Functor;
    use crate::specs::monad::infix::Monad;
    use crate::standard::result::ResultK;

    impl<'a, A: 'a, E> HKP<'a> for Result<A, E> {
        type T<B: 'a> = Result<B, E>;
    }

    impl<'a, A: 'a, E> Transform<'a, A> for Result<A, E> {
        type This = ResultK<E>;

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

    impl<'a, A: 'a, E> Functor<'a, A> for Result<A, E> {
        type ThisL = ResultK<E>;
        type TL<B: 'a> = Result<B, E>;
    }

    impl<'a, A: 'a, E> Applicative<'a, A> for Result<A, E> {
        type ThisL = ResultK<E>;
        type TL<B: 'a> = Result<B, E>;
    }

    impl<'a, A: 'a, E: 'a> Bind<'a, A> for Result<A, E> {
        type ThisL = ResultK<E>;
        type TL<B: 'a> = Result<B, E>;
    }

    impl<'a, A: 'a, E: 'a> Monad<'a, A> for Result<A, E> {
        type ThisL = ResultK<E>;
        type TL<B: 'a> = Result<B, E>;
    }
}
