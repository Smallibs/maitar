/*
 * MIT License
 *
 * Copyright (c) 2023-2024 Didier Plaindoux
 */

#[cfg(test)]
mod tests_join {

    use maitar::specs::bind::Bind;
    use maitar::standard::option::OptionK;
    use maitar::standard::result::ResultK;
    fn test_join<'a, This: Bind<'a>>(mma: This::T<This::T<i32>>) -> This::T<i32> {
        This::join(mma)
    }

    #[test]
    fn join_option_some_some() {
        type This = OptionK;
        assert_eq!(test_join::<This>(Some(Some(1))), Some(1))
    }

    #[test]
    fn join_option_some_none() {
        type This = OptionK;
        assert_eq!(test_join::<This>(Some(None)), None)
    }

    #[test]
    fn join_option_none() {
        type This = OptionK;
        assert_eq!(test_join::<This>(None), None)
    }

    #[test]
    fn join_result_ok_ok() {
        type This = ResultK<&'static str>;
        assert_eq!(test_join::<This>(Ok(Ok(1))), Ok(1))
    }

    #[test]
    fn join_result_ok_err() {
        type This = ResultK<&'static str>;
        assert_eq!(test_join::<This>(Ok(Err(""))), Err(""))
    }

    #[test]
    fn join_result_err() {
        type This = ResultK<&'static str>;
        assert_eq!(test_join::<This>(Err("")), Err(""))
    }
}

#[cfg(test)]
mod tests_bind {
    use maitar::specs::bind::Bind;
    use maitar::standard::option::OptionK;
    use maitar::standard::result::ResultK;
    use maitar::standard::vec::VecK;

    fn test_bind<'a, This: Bind<'a> + 'a>(
        ma: This::T<i32>,
        f: fn(i32) -> This::T<i32>,
    ) -> This::T<i32> {
        This::bind(ma, f)
    }

    #[test]
    fn bind_option_some() {
        type This = OptionK;
        assert_eq!(test_bind::<This>(Some(1), |a| Some(a + 1)), Some(2))
    }

    #[test]
    fn bind_option_none() {
        type This = OptionK;
        assert_eq!(test_bind::<This>(None, |a| Some(a + 1)), None)
    }

    #[test]
    fn bind_result_ok() {
        type This = ResultK<&'static str>;
        assert_eq!(test_bind::<This>(Ok(1), |a| Ok(a + 1)), Ok(2))
    }

    #[test]
    fn bind_result_err() {
        type This = ResultK<&'static str>;
        assert_eq!(test_bind::<This>(Err("Error"), |a| Ok(a + 1)), Err("Error"))
    }

    #[test]
    fn bind_vec() {
        type This = VecK;
        assert_eq!(test_bind::<This>(vec![1], |a| vec![a + 1]), vec![2])
    }

    mod infix {
        use maitar::core::hkp::HKP;
        use maitar::specs::bind::infix::Bind;

        type Int2Int<'a, This> = fn(i32) -> <This as HKP<'a>>::T<i32>;

        fn test_bind_with_f<'a, This: Bind<'a, i32, TL<i32> = This> + 'a>(
            f: Int2Int<'a, This>,
            g: Int2Int<'a, This>,
            ma: This,
        ) -> This::T<i32> {
            ma.bind::<i32, Int2Int<'a, This>>(f)
                .bind::<i32, Int2Int<'a, This>>(g)
        }

        #[test]
        fn bind_option_some() {
            type This = Option<i32>;
            assert_eq!(
                test_bind_with_f::<This>(|i| Some(i - 1), |i| Some(i + 2), Some(1)),
                Some(2)
            )
        }

        #[test]
        fn bind_result_ok() {
            type This = Result<i32, &'static str>;
            assert_eq!(
                test_bind_with_f::<This>(|i| Ok(i - 1), |i| Ok(i + 2), Ok(1)),
                Ok(2)
            )
        }
    }
}
