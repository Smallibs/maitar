#[cfg(test)]
mod tests_apply {
    use maitar::specs::applicative::Applicative;
    use maitar::standard::option::OptionK;
    use maitar::standard::result::ResultK;

    fn test_apply<This: Applicative>(ma: This::T<i32>) -> This::T<i32> {
        This::apply(This::pure(|i| i + 1), ma)
    }

    fn test_apply_with_f<This: Applicative>(
        f: This::T<fn(i32) -> i32>,
        ma: This::T<i32>,
    ) -> This::T<i32> {
        This::apply(f, ma)
    }

    #[test]
    fn map_option_some() {
        type This = OptionK;
        assert_eq!(test_apply::<This>(Some(1)), Some(2))
    }

    #[test]
    fn map_option_none() {
        type This = OptionK;
        assert_eq!(test_apply::<This>(None), None)
    }

    #[test]
    fn map_option_some_with_f() {
        type This = OptionK;
        assert_eq!(test_apply_with_f::<This>(None, Some(1)), None)
    }

    #[test]
    fn map_result_ok() {
        type This = ResultK<&'static str>;
        assert_eq!(test_apply::<This>(Ok(1)), Ok(2))
    }

    #[test]
    fn map_result_ok_with_f() {
        type This = ResultK<&'static str>;
        assert_eq!(test_apply_with_f::<This>(Err(""), Ok(1)), Err(""))
    }

    #[test]
    fn map_result_err() {
        type This = ResultK<&'static str>;
        assert_eq!(test_apply::<This>(Err("Error")), Err("Error"))
    }
}
