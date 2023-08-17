use std::marker::PhantomData;

use crate::core::hkp::HKP;
use crate::specs::applicative::Applicative;
use crate::specs::bind::Bind;
use crate::specs::functor::Functor;
use crate::specs::monad::Monad;

pub struct ResultK<E> {
    _ignore: PhantomData<E>,
}

impl<E> HKP for ResultK<E> {
    type T<A> = Result<A, E>;
}

impl<E> Functor for ResultK<E> {
    fn map<A, B, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B,
    {
        ma.map(f)
    }
}

impl<E> Applicative for ResultK<E> {
    fn pure<A>(a: A) -> Self::T<A> {
        Ok(a)
    }

    fn apply<A, B, MAP>(mf: Self::T<MAP>, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B,
    {
        match mf {
            Ok(f) => Self::map(f, ma),
            Err(e) => Err(e),
        }
    }
}

impl<E> Bind for ResultK<E> {
    fn join<A>(mma: Self::T<Self::T<A>>) -> Self::T<A> {
        match mma {
            Ok(ma) => ma,
            Err(e) => Err(e),
        }
    }
}

impl<E> Monad for ResultK<E> {}

mod infix {
    use crate::core::hkp::HKP;
    use crate::specs::applicative::infix::Applicative;
    use crate::specs::bind::infix::Bind;
    use crate::specs::functor::infix::Functor;
    use crate::core::transform::Transform;
    use crate::specs::monad::infix::Monad;
    use crate::standard::result::ResultK;

    impl<A, E> Transform<A> for Result<A, E> {
        type This = ResultK<E>;
        type T<B> = Result<B, E>;

        fn from_hkp<B>(a: <Self::This as HKP>::T<B>) -> Self::T<B> {
            a
        }

        fn from_self<B>(a: Self::T<B>) -> <Self::This as HKP>::T<B> {
            a
        }

        fn to_hkp(self) -> <Self::This as HKP>::T<A> {
            self
        }
    }

    impl<A, E> Functor<A> for Result<A, E> {
        type ThisL = ResultK<E>;
        type TL<B> = Result<B, E>;
    }

    impl<A, E> Applicative<A> for Result<A, E> {
        type ThisL = ResultK<E>;
        type TL<B> = Result<B, E>;
    }

    impl<A, E> Bind<A> for Result<A, E> {
        type ThisL = ResultK<E>;
        type TL<B> = Result<B, E>;
    }

    impl<A, E> Monad<A> for Result<A, E> {
        type ThisL = ResultK<E>;
        type TL<B> = Result<B, E>;
    }
}
