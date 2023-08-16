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
    fn map<A, B>(f: fn(A) -> B, ma: Self::T<A>) -> Self::T<B> {
        ma.map(f)
    }
}

impl<E> Applicative for ResultK<E> {
    fn pure<A>(a: A) -> Self::T<A> {
        Ok(a)
    }

    fn apply<A, B>(mf: Self::T<fn(A) -> B>, ma: Self::T<A>) -> Self::T<B> {
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
    use crate::specs::applicative::infix::Applicative as InfixApplicative;
    use crate::specs::applicative::Applicative;
    use crate::specs::bind::infix::Bind as InfixBind;
    use crate::specs::bind::Bind;
    use crate::specs::functor::infix::Functor as InfixFunctor;
    use crate::specs::functor::Functor;
    use crate::standard::result::ResultK;

    impl<A, E> InfixFunctor<A> for Result<A, E> {
        type T<B> = Result<B, E>;

        fn map<B>(self, f: fn(A) -> B) -> Self::T<B> {
            <ResultK<E>>::map(f, self)
        }
    }

    impl<A, E> InfixApplicative<A> for Result<A, E> {
        type This = ResultK<E>;
        type T<B> = Result<B, E>;

        fn from<B>(a: <Self::This as HKP>::T<B>) -> Self::T<B> {
            a
        }

        fn to(self) -> <Self::This as HKP>::T<A> {
            self
        }

        fn apply<B>(self, mf: Self::T<fn(A) -> B>) -> Self::T<B> {
            <ResultK<E>>::apply(mf, self)
        }
    }

    impl<A, E> InfixBind<A> for Result<A, E> {
        type This = ResultK<E>;
        type T<B> = Result<B, E>;

        fn from<B>(a: <Self::This as HKP>::T<B>) -> Self::T<B> {
            a
        }

        fn to(self) -> <Self::This as HKP>::T<A> {
            self
        }

        fn bind<B>(self, mf: fn(A) -> Self::T<B>) -> Self::T<B> {
            <ResultK<E>>::bind(self, mf)
        }
    }
}
