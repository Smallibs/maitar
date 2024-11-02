/*
 * MIT License
 *
 * Copyright (c) 2023-2024 Didier Plaindoux
 */

pub trait HKP<'a> {
    type T<A: 'a>;
}
