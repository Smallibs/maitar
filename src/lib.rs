// -------------------------------------------------------------------------------------------------

mod core;
mod specs;
mod standard;

#[cfg(test)]
mod tests {
    use crate::specs::monad::Monad;
    use crate::standard::result::ResultK;

    #[test]
    fn it_works() {
        let ma = Ok(1);
        let mb = <ResultK<String> as Monad>::bind(|a| Ok(a + 1), ma);
        assert_eq!(mb, Ok(2))
    }
}
