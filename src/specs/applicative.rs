use crate::specs::functor::Functor;

pub trait Applicative<'a>: Functor<'a> {
    fn pure<A>(a: A) -> Self::T<A>;

    fn apply<A, B, MAP>(mf: Self::T<MAP>, ma: Self::T<A>) -> Self::T<B>
    where
        A: Clone,
        MAP: Fn(A) -> B;
}

pub mod infix {
    use crate::core::transform::Transform;
    use crate::specs::applicative::Applicative as Api;

    pub trait Applicative<'a, A: 'a>:
        Transform<'a, A, T<A> = Self::TL<A>, This = Self::ThisL>
    {
        type ThisL: Api<'a>;
        type TL<B: 'a>: Applicative<'a, B>;

        fn pure<B>(a: B) -> Self::T<B> {
            Self::hkp_to_self(Self::This::pure(a))
        }

        fn apply<B, MAP>(self, mf: Self::T<MAP>) -> Self::T<B>
        where
            A: Clone,
            MAP: Fn(A) -> B,
            Self: Sized,
        {
            Self::hkp_to_self(Self::This::apply(
                Self::self_to_hkp::<MAP>(mf),
                self.to_hkp(),
            ))
        }
    }
}
