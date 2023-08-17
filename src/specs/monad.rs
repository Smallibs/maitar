use crate::specs::bind::Bind;

pub trait Monad: Bind {
    fn returns<A>(a: A) -> Self::T<A> {
        Self::pure(a)
    }
}

pub mod infix {
    use crate::core::transform::Transform;
    use crate::specs::bind::Bind;
    use crate::specs::monad::Monad as Api;

    pub trait Monad<A>: Transform<A, T<A> = Self::TL<A>, This = Self::ThisL> {
        type ThisL: Api;

        type TL<B>: Monad<B>;

        fn returns<B>(b: B) -> Self::T<B> {
            Self::from_hkp(Self::This::returns(b))
        }

        fn join<B>(mma: Self::T<Self::T<B>>) -> Self::T<B> {
            Self::from_hkp(<Self::This as Bind>::bind(Self::from_self(mma), |a| {
                Self::from_self::<B>(a)
            }))
        }

        fn bind<B, BIND>(self, mf: BIND) -> Self::T<B>
        where
            BIND: Fn(A) -> Self::T<B>,
            Self: Sized,
        {
            Self::from_hkp(<Self::This as Bind>::bind(self.to_hkp(), |a| {
                Self::from_self::<B>(mf(a))
            }))
        }
    }
}
