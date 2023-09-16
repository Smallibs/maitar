use std::marker::PhantomData;

use crate::core::hkp::HKP;
use crate::specs::applicative::Applicative;
use crate::specs::bind::Bind;
use crate::specs::functor::Functor;
use crate::specs::monad::Monad;
use crate::standard::either::Either::{Left, Right};

pub struct EitherK<L> {
    _ignore: PhantomData<L>,
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<'a, L> HKP<'a> for EitherK<L> {
    type T<R: 'a> = Either<L, R>;
}

impl<'a, E> Functor<'a> for EitherK<E> {
    fn map<A: 'a, B: 'a, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B + 'a,
    {
        match ma {
            Left(l) => Left(l),
            Right(r) => Right(f(r)),
        }
    }
}

impl<'a, E> Applicative<'a> for EitherK<E> {
    fn pure<A: 'a>(a: A) -> Self::T<A> {
        Right(a)
    }

    fn apply<A: 'a, B: 'a, MAP>(mf: Self::T<MAP>, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B + 'a,
    {
        match mf {
            Left(l) => Left(l),
            Right(f) => Self::map(f, ma),
        }
    }
}

impl<'a, E: 'a> Bind<'a> for EitherK<E> {
    fn bind<A: 'a, B: 'a, BIND>(ma: Self::T<A>, f: BIND) -> Self::T<B>
    where
        BIND: Fn(A) -> Self::T<B> + 'a,
    {
        match ma {
            Left(a) => Left(a),
            Right(r) => f(r),
        }
    }
}

impl<'a, E: 'a> Monad<'a> for EitherK<E> {}

mod infix {
    use crate::core::hkp::HKP;
    use crate::core::transform::Transform;
    use crate::specs::applicative::infix::Applicative;
    use crate::specs::bind::infix::Bind;
    use crate::specs::functor::infix::Functor;
    use crate::specs::monad::infix::Monad;
    use crate::standard::either::{Either, EitherK};

    impl<'a, L, R> HKP<'a> for Either<L, R> {
        type T<B: 'a> = Either<L, B>;
    }

    impl<'a, L, R: 'a> Transform<'a, R> for Either<L, R> {
        type This = EitherK<L>;

        fn hkp_to_self<B: 'a>(a: <Self::This as HKP<'a>>::T<B>) -> Self::T<B> {
            a
        }

        fn self_to_hkp<B: 'a>(a: Self::T<B>) -> <Self::This as HKP<'a>>::T<B> {
            a
        }

        fn to_hkp(self) -> <Self::This as HKP<'a>>::T<R> {
            self
        }
    }

    impl<'a, L, R: 'a> Functor<'a, R> for Either<L, R> {
        type ThisL = EitherK<L>;
        type TL<B: 'a> = Either<L, B>;
    }

    impl<'a, L, R: 'a> Applicative<'a, R> for Either<L, R> {
        type ThisL = EitherK<L>;
        type TL<B: 'a> = Either<L, B>;
    }

    impl<'a, L: 'a, R: 'a> Bind<'a, R> for Either<L, R> {
        type ThisL = EitherK<L>;
        type TL<B: 'a> = Either<L, B>;
    }

    impl<'a, L: 'a, R: 'a> Monad<'a, R> for Either<L, R> {
        type ThisL = EitherK<L>;
        type TL<B: 'a> = Either<L, B>;
    }
}
