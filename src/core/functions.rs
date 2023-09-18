/*
 * MIT License
 *
 * Copyright (c) 2023 Didier Plaindoux
 */

use crate::core::types::FunOnceLT;

pub fn uncurry<'a, A, B, C, F>(f: F) -> Box<dyn Fn(A, B) -> C + 'a>
where
    F: Fn(A) -> Box<dyn Fn(B) -> C> + 'a,
{
    Box::new(move |a, b| f(a)(b))
}

pub fn curry<'a, A, B, C, F>(f: F) -> FunOnceLT<'a, A, FunOnceLT<'a, B, C>>
where
    A: 'a,
    F: FnOnce(A, B) -> C + 'a,
{
    Box::new(move |a| Box::new(move |b| f(a, b)))
}

pub fn curry3<'a, A, B, C, D, F>(f: F) -> FunOnceLT<'a, A, FunOnceLT<'a, B, FunOnceLT<'a, C, D>>>
where
    A: 'a,
    B: 'a,
    F: FnOnce(A, B, C) -> D + 'a,
{
    Box::new(move |a| Box::new(move |b| Box::new(move |c| f(a, b, c))))
}
