use crate::specs::bind::Bind;

pub trait Monad: Bind {
    fn returns<A>(a: A) -> Self::T<A> {
        Self::pure(a)
    }
}

pub mod infix {
    use crate::core::transform::Transform;
    use crate::specs::monad::Monad as Api;

    pub trait Monad<A>: Transform<A, T<A> = Self::TL<A>, This = Self::ThisL> {
        type ThisL: Api;

        type TL<B>: Monad<B>;

        fn returns<B>(b: B) -> Self::T<B> {
            Self::from_hkp(Self::This::returns(b))
        }
    }
}
