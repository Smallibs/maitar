use crate::core::hkp::HKP;

pub trait Functor: HKP {
    fn map<A, B, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
        where
            MAP: Fn(A) -> B;
}

pub mod infix {
    use crate::core::transform::Transform;
    use crate::specs::functor::Functor as Api;

    pub trait Functor<A>: Transform<A, T<A>=Self::TL<A>, This=Self::ThisL> {
        type ThisL: Api;
        type TL<B>: Functor<B>;

        fn map<B, MAP>(self, f: MAP) -> Self::T<B>
            where
                MAP: Fn(A) -> B,
                Self: Sized,
        {
            Self::from_hkp(Self::This::map(f, self.to_hkp()))
        }
    }
}
