// -------------------------------------------------------------------------------------------------
mod specs;
mod standard;

#[cfg(test)]
mod tests {
    use crate::specs::monad::Monad;
    use crate::standard::option::OptionK;

    #[test]
    fn it_works() {
        let ma = Some(1);
        let mb = OptionK::bind(|a| { Some(a + 1) }, ma);
        assert_eq!(mb, Some(2))
    }
}
