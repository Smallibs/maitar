use rustine::specs::applicative::Applicative;
use rustine::specs::monad::Monad;

trait Test<This: Monad + Applicative> {
    fn test01() -> This::T<i32> {
        let ma = This::pure(1);
        This::bind(|a| This::pure(a + 1), ma)
    }
}

#[cfg(test)]
mod tests {
    use rustine::specs::applicative::Applicative;
    use rustine::standard::option::OptionK;
    use rustine::standard::result::ResultK;
    use crate::Test;


    #[test]
    fn it_works_for_option() {
        type This = OptionK;
        impl Test<This> for This {}
        let mb = This::test01();
        assert_eq!(mb, This::pure(2))
    }

    #[test]
    fn it_works_for_result() {
        type This = ResultK<String>;
        impl Test<This> for This {}
        let mb = This::test01();
        assert_eq!(mb, This::pure(2))
    }
}
