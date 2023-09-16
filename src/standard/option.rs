use crate::core::hkp::HKP;
use crate::specs::applicative::Applicative;
use crate::specs::bind::Bind;
use crate::specs::functor::Functor;
use crate::specs::monad::Monad;

pub struct OptionK;

impl<'a> HKP<'a> for OptionK {
    type T<A: 'a> = Option<A>;
}

impl<'a> Functor<'a> for OptionK {
    fn map<A: 'a, B: 'a, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B,
    {
        ma.map(f)
    }
}

impl<'a> Applicative<'a> for OptionK {
    fn pure<A: 'a>(a: A) -> Self::T<A> {
        Some(a)
    }

    fn apply<A: 'a, B: 'a, MAP>(mf: Self::T<MAP>, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B + 'a,
    {
        match mf {
            Some(f) => Self::map(f, ma),
            None => None,
        }
    }
}

impl<'a> Bind<'a> for OptionK {
    fn bind<A: 'a, B: 'a, BIND>(ma: Self::T<A>, f: BIND) -> Self::T<B>
    where
        BIND: Fn(A) -> Self::T<B> + 'a,
    {
        match ma {
            Some(a) => f(a),
            None => None,
        }
    }
}

impl Monad<'_> for OptionK {}

pub mod infix {
    use crate::core::hkp::HKP;
    use crate::core::transform::Transform;
    use crate::specs::applicative::infix::Applicative;
    use crate::specs::bind::infix::Bind;
    use crate::specs::functor::infix::Functor;
    use crate::specs::monad::infix::Monad;
    use crate::standard::option::OptionK;

    impl<'a, A> HKP<'a> for Option<A> {
        type T<B: 'a> = Option<B>;
    }

    impl<'a, A: 'a> Transform<'a, A> for Option<A> {
        type This = OptionK;

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

    impl<'a, A: 'a> Functor<'a, A> for Option<A> {
        type ThisL = OptionK;
        type TL<B: 'a> = Option<B>;
    }

    impl<'a, A: 'a> Applicative<'a, A> for Option<A> {
        type ThisL = OptionK;
        type TL<B: 'a> = Option<B>;
    }

    impl<'a, A: 'a> Bind<'a, A> for Option<A> {
        type ThisL = OptionK;
        type TL<B: 'a> = Option<B>;
    }

    impl<'a, A: 'a> Monad<'a, A> for Option<A> {
        type ThisL = OptionK;
        type TL<B: 'a> = Option<B>;
    }
}
