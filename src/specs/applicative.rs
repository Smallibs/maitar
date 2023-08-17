use crate::specs::functor::Functor;

pub trait Applicative: Functor {
    fn pure<A>(a: A) -> Self::T<A>;

    fn apply<A, B, MAP>(mf: Self::T<MAP>, ma: Self::T<A>) -> Self::T<B>
    where
        MAP: Fn(A) -> B;
}

pub mod infix {
    use crate::core::transform::Transform;
    use crate::specs::applicative::Applicative as Api;

    pub trait Applicative<A>: Transform<A, T<A> = Self::TL<A>, This = Self::ThisL> {
        type ThisL: Api;
        type TL<B>: Applicative<B>;

        fn pure<B>(a: B) -> Self::T<B> {
            Self::from_hkp(Self::This::pure(a))
        }

        fn apply<B, MAP>(self, mf: Self::T<MAP>) -> Self::T<B>
        where
            MAP: Fn(A) -> B,
            Self: Sized,
        {
            Self::from_hkp(Self::This::apply(Self::from_self::<MAP>(mf), self.to_hkp()))
        }
    }
}
