#[cfg(test)]
mod tests_returns {
    use maitar::specs::monad::Monad;
    use maitar::standard::option::OptionK;
    use maitar::standard::result::ResultK;

    fn test_join<This: Monad>(mma: This::T<This::T<i32>>) -> This::T<i32> {
        This::join(mma)
    }

    #[test]
    fn join_option_some_some() {
        type This = OptionK;
        assert_eq!(
            test_join::<This>(This::returns(This::returns(1))),
            This::returns(1)
        )
    }

    #[test]
    fn join_option_some_none() {
        type This = OptionK;
        assert_eq!(test_join::<This>(This::returns(None)), None)
    }

    #[test]
    fn join_option_none() {
        type This = OptionK;
        assert_eq!(test_join::<This>(None), None)
    }

    #[test]
    fn join_result_ok_ok() {
        type This = ResultK<&'static str>;
        assert_eq!(
            test_join::<This>(This::returns(This::returns(1))),
            This::returns(1)
        )
    }

    #[test]
    fn join_result_ok_err() {
        type This = ResultK<&'static str>;
        assert_eq!(test_join::<This>(This::returns(Err(""))), Err(""))
    }

    #[test]
    fn join_result_err() {
        type This = ResultK<&'static str>;
        assert_eq!(test_join::<This>(Err("")), Err(""))
    }
}

#[cfg(test)]
mod tests_bind {
    use maitar::specs::monad::Monad;
    use maitar::standard::option::OptionK;
    use maitar::standard::result::ResultK;

    fn test_bind<This: Monad>(ma: This::T<i32>) -> This::T<i32> {
        This::bind(ma, |a| This::returns(a + 1))
    }

    #[test]
    fn bind_option_some() {
        type This = OptionK;
        assert_eq!(test_bind::<This>(This::returns(1)), This::returns(2))
    }

    #[test]
    fn bind_option_none() {
        type This = OptionK;
        assert_eq!(test_bind::<This>(None), None)
    }

    #[test]
    fn bind_result_ok() {
        type This = ResultK<&'static str>;
        assert_eq!(test_bind::<This>(This::returns(1)), This::returns(2))
    }

    #[test]
    fn bind_result_err() {
        type This = ResultK<&'static str>;
        assert_eq!(test_bind::<This>(Err("Error")), Err("Error"))
    }

    mod infix {
        use maitar::specs::monad::infix::Monad;

        fn test_bind_with_f<This: Monad<i32, TL<i32> = This>>(ma: This) -> This::T<i32> {
            ma.bind::<i32, _>(move |a| This::returns(a - 1))
                .bind::<i32, _>(move |a| This::returns(a + 2))
        }

        #[test]
        fn apply_option_some() {
            type This = Option<i32>;
            assert_eq!(test_bind_with_f::<This>(Some(1)), Some(2))
        }

        #[test]
        fn apply_result_ok() {
            type This = Result<i32, &'static str>;
            assert_eq!(test_bind_with_f::<This>(Ok(1)), Ok(2))
        }
    }
}
