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
    fn map<A, B, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B,
    {
        ma.map(f)
    }
}

impl Applicative for OptionK {
    fn pure<A>(a: A) -> Self::T<A> {
        Some(a)
    }

    fn apply<A, B, MAP>(mf: Self::T<MAP>, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B,
    {
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
    use crate::core::transform::Transform;
    use crate::specs::applicative::infix::Applicative;
    use crate::specs::bind::infix::Bind;
    use crate::specs::functor::infix::Functor;
    use crate::specs::monad::infix::Monad;
    use crate::standard::option::OptionK;

    impl<A> HKP for Option<A> {
        type T<B> = Option<B>;
    }

    impl<A> Transform<A> for Option<A> {
        type This = OptionK;

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

    impl<A> Functor<A> for Option<A> {
        type ThisL = OptionK;
        type TL<B> = Option<B>;
    }

    impl<A> Applicative<A> for Option<A> {
        type ThisL = OptionK;
        type TL<B> = Option<B>;
    }

    impl<A> Bind<A> for Option<A> {
        type ThisL = OptionK;
        type TL<B> = Option<B>;
    }

    impl<A> Monad<A> for Option<A> {
        type ThisL = OptionK;
        type TL<B> = Option<B>;
    }
}
