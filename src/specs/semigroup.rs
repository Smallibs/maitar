/*
 * MIT License
 *
 * Copyright (c) 2023 Didier Plaindoux
 */

pub trait Semigroup {
    type T;
    fn compose(a: Self::T, b: Self::T) -> Self::T;
}
