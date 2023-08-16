use crate::core::hkp::HKP;
use crate::specs::applicative::Applicative;
use crate::specs::bind::Bind;
use crate::specs::functor::Functor;
use crate::specs::monad::Monad;

pub struct OptionK;

impl HKP for OptionK {
    type T<A> = Option<A>;
}

impl Functor for OptionK {
    fn map<A, B>(f: fn(A) -> B, ma: Self::T<A>) -> Self::T<B> {
        ma.map(f)
    }
}

impl Applicative for OptionK {
    fn pure<A>(a: A) -> Self::T<A> {
        Some(a)
    }

    fn apply<A, B>(mf: Self::T<fn(A) -> B>, ma: Self::T<A>) -> Self::T<B> {
        match mf {
            Some(f) => Self::map(f, ma),
            None => None,
        }
    }
}

impl Bind for OptionK {
    fn join<A>(mma: Self::T<Self::T<A>>) -> Self::T<A> {
        match mma {
            Some(ma) => ma,
            None => None,
        }
    }
}

impl Monad for OptionK {}

pub mod infix {
    use crate::core::hkp::HKP;
    use crate::specs::applicative::Applicative;
    use crate::specs::applicative::infix::Applicative as InfixApplicative;
    use crate::specs::bind::Bind;
    use crate::specs::bind::infix::Bind as InfixBind;
    use crate::specs::functor::Functor;
    use crate::specs::functor::infix::Functor as InfixFunctor;
    use crate::standard::option::OptionK;

    impl<A> InfixFunctor<A> for Option<A> {
        type T<B> = Option<B>;

        fn map<B>(self, f: fn(A) -> B) -> Self::T<B> {
            OptionK::map(f, self)
        }
    }

    impl<A> InfixApplicative<A, OptionK> for Option<A> {
        type T<B> = Option<B>;

        fn from<B>(a: <OptionK as HKP>::T<B>) -> Self::T<B> {
            a
        }

        fn to(self) -> <OptionK as HKP>::T<A> {
            self
        }

        fn apply<B>(self, mf: Self::T<fn(A) -> B>) -> Self::T<B> {
            OptionK::apply(mf, self)
        }
    }

    impl<A> InfixBind<A, OptionK> for Option<A> {
        type T<B> = Option<B>;

        fn this<B>(a: <OptionK as HKP>::T<B>) -> Self::T<B> {
            a
        }

        fn bind<B>(self, mf: fn(A) -> Self::T<B>) -> Self::T<B> {
            OptionK::bind(self, mf)
        }
    }
}
