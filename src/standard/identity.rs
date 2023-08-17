use crate::core::hkp::HKP;
use crate::specs::applicative::Applicative;
use crate::specs::bind::Bind;
use crate::specs::functor::Functor;
use crate::specs::monad::Monad;

pub struct IdentityK;

pub struct Identity<A> {
    value: A,
}

impl HKP for IdentityK {
    type T<A> = Identity<A>;
}

impl Functor for IdentityK {
    fn map<A, B, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B,
    {
        Identity { value: f(ma.value) }
    }
}

impl Applicative for IdentityK {
    fn pure<A>(a: A) -> Self::T<A> {
        Identity { value: a }
    }

    fn apply<A, B, MAP>(mf: Self::T<MAP>, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B,
    {
        Self::map(mf.value, ma)
    }
}

impl Bind for IdentityK {
    fn join<A>(mma: Self::T<Self::T<A>>) -> Self::T<A> {
        mma.value
    }
}

impl Monad for IdentityK {}

pub mod infix {
    use crate::core::hkp::HKP;
    use crate::core::transform::Transform;
    use crate::specs::applicative::infix::Applicative;
    use crate::specs::bind::infix::Bind;
    use crate::specs::functor::infix::Functor;
    use crate::specs::monad::infix::Monad;
    use crate::standard::identity::{Identity, IdentityK};

    impl<A> HKP for Identity<A> {
        type T<B> = Identity<B>;
    }

    impl<A> Transform<A> for Identity<A> {
        type This = IdentityK;

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

    impl<A> Functor<A> for Identity<A> {
        type ThisL = IdentityK;
        type TL<B> = Identity<B>;
    }

    impl<A> Applicative<A> for Identity<A> {
        type ThisL = IdentityK;
        type TL<B> = Identity<B>;
    }

    impl<A> Bind<A> for Identity<A> {
        type ThisL = IdentityK;
        type TL<B> = Identity<B>;
    }

    impl<A> Monad<A> for Identity<A> {
        type ThisL = IdentityK;
        type TL<B> = Identity<B>;
    }
}
