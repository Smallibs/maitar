use crate::specs::applicative::Applicative;

pub trait Bind<'a>: Applicative<'a>
where
    Self: 'a,
{
    fn join<A>(mma: Self::T<Self::T<A>>) -> Self::T<A>;

    fn bind<A, B, BIND>(ma: Self::T<A>, mf: BIND) -> Self::T<B>
    where
        BIND: Fn(A) -> Self::T<B> + 'a,
    {
        Self::join(Self::map(mf, ma))
    }
}

pub mod infix {
    use crate::core::transform::Transform;
    use crate::specs::bind::Bind as Api;

    pub trait Bind<'a, A: 'a>: Transform<'a, A, T<A> = Self::TL<A>, This = Self::ThisL> {
        type ThisL: Api<'a>;
        type TL<B: 'a>: Bind<'a, B>;

        fn join<B>(mma: Self::T<Self::T<B>>) -> Self::T<B> {
            Self::hkp_to_self(Self::This::bind(Self::self_to_hkp(mma), |a| {
                Self::self_to_hkp::<B>(a)
            }))
        }

        fn bind<B, BIND>(self, mf: BIND) -> Self::T<B>
        where
            BIND: Fn(A) -> Self::T<B> + 'a,
            Self: Sized,
        {
            Self::hkp_to_self(Self::This::bind(self.to_hkp(), move |a| {
                Self::self_to_hkp::<B>(mf(a))
            }))
        }
    }
}
