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

impl<L> HKP for EitherK<L> {
    type T<R> = Either<L, R>;
}

impl<E> Functor for EitherK<E> {
    fn map<A, B, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B,
    {
        match ma {
            Left(l) => Left(l),
            Right(r) => Right(f(r)),
        }
    }
}

impl<E> Applicative for EitherK<E> {
    fn pure<A>(a: A) -> Self::T<A> {
        Right(a)
    }

    fn apply<A, B, MAP>(mf: Self::T<MAP>, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B,
    {
        match mf {
            Left(l) => Left(l),
            Right(f) => Self::map(f, ma),
        }
    }
}

impl<E> Bind for EitherK<E> {
    fn join<A>(mma: Self::T<Self::T<A>>) -> Self::T<A> {
        match mma {
            Left(ma) => Left(ma),
            Right(r) => r,
        }
    }
}

impl<E> Monad for EitherK<E> {}

mod infix {
    use crate::core::hkp::HKP;
    use crate::core::transform::Transform;
    use crate::specs::applicative::infix::Applicative;
    use crate::specs::bind::infix::Bind;
    use crate::specs::functor::infix::Functor;
    use crate::specs::monad::infix::Monad;
    use crate::standard::either::{Either, EitherK};

    impl<L, R> HKP for Either<L, R> {
        type T<B> = Either<L, B>;
    }

    impl<L, R> Transform<R> for Either<L, R> {
        type This = EitherK<L>;

        fn from_hkp<B>(a: <Self::This as HKP>::T<B>) -> Self::T<B> {
            a
        }

        fn from_self<B>(a: Self::T<B>) -> <Self::This as HKP>::T<B> {
            a
        }

        fn to_hkp(self) -> <Self::This as HKP>::T<R> {
            self
        }
    }

    impl<L, R> Functor<R> for Either<L, R> {
        type ThisL = EitherK<L>;
        type TL<B> = Either<L, B>;
    }

    impl<L, R> Applicative<R> for Either<L, R> {
        type ThisL = EitherK<L>;
        type TL<B> = Either<L, B>;
    }

    impl<L, R> Bind<R> for Either<L, R> {
        type ThisL = EitherK<L>;
        type TL<B> = Either<L, B>;
    }

    impl<L, R> Monad<R> for Either<L, R> {
        type ThisL = EitherK<L>;
        type TL<B> = Either<L, B>;
    }
}
