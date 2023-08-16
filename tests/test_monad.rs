#[cfg(test)]
mod tests_join {
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
        This::bind(|a| This::returns(a + 1), ma)
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
}
