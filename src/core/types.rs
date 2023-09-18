/*
 * MIT License
 *
 * Copyright (c) 2023 Didier Plaindoux
 */

pub type Fun<A, B> = Box<dyn Fn(A) -> B>;
pub type Fun2<A, B, C> = Box<dyn Fn(A, B) -> C>;
pub type FunLT<'a, A, B> = Box<dyn Fn(A) -> B + 'a>;
pub type FunOnce<A, B> = Box<dyn FnOnce(A) -> B>;
pub type FunOnceLT<'a, A, B> = Box<dyn FnOnce(A) -> B + 'a>;
pub type FunOnce2LT<'a, A, B, C> = Box<dyn FnOnce(A, B) -> C + 'a>;
