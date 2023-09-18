/*
 * MIT License
 *
 * Copyright (c) 2023 Didier Plaindoux
 */

pub type Fun<A, B> = Box<dyn Fn(A) -> B>;
pub type FunOnce<'a, A, B> = Box<dyn FnOnce(A) -> B + 'a>;
