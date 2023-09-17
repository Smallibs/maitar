/*
 * MIT License
 *
 * Copyright (c) 2023 Didier Plaindoux
 */

use crate::specs::semigroup::Semigroup;

pub trait Monoid: Semigroup {
    fn neutral() -> Self::T;
}
