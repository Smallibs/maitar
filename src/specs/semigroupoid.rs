/*
 * MIT License
 *
 * Copyright (c) 2023-2024 Didier Plaindoux
 */

use crate::core::morphism::Morphism;

pub trait Semigroupoid: Morphism {
    /*
     * Law:
     * - compose(f, compose(g,h)) == compose(compose(f,g), h)
     */
    fn compose<A, B, C>(f: Self::T<B, C>, g: Self::T<A, B>) -> Self::T<A, C>;
}
