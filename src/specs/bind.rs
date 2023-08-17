use crate::specs::applicative::Applicative;

pub trait Bind: Applicative {
    fn join<A>(mma: Self::T<Self::T<A>>) -> Self::T<A>;

    fn bind<A, B, BIND>(ma: Self::T<A>, mf: BIND) -> Self::T<B>
    where
        BIND: Fn(A) -> Self::T<B>,
    {
        Self::join(Self::map(mf, ma))
    }
}

pub mod infix {
    use crate::core::transform::Transform;
    use crate::specs::bind::Bind as Api;

    pub trait Bind<A>: Transform<A, T<A> = Self::TL<A>, This = Self::ThisL> {
        type ThisL: Api;
        type TL<B>: Bind<B>;

        fn join<B>(mma: Self::T<Self::T<B>>) -> Self::T<B> {
            Self::from_hkp(Self::This::bind(Self::from_self(mma), |a| {
                Self::from_self::<B>(a)
            }))
        }

        fn bind<B, BIND>(self, mf: BIND) -> Self::T<B>
        where
            BIND: Fn(A) -> Self::T<B>,
            Self: Sized,
        {
            Self::from_hkp(Self::This::bind(self.to_hkp(), |a| {
                Self::from_self::<B>(mf(a))
            }))
        }
    }
}
