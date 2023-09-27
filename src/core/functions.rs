/*
 * MIT License
 *
 * Copyright (c) 2023 Didier Plaindoux
 */

pub fn uncurry<'a, A, B, C, F, G>(f: F) -> impl Fn(A, B) -> C + 'a
where
    F: Fn(A) -> G + 'a,
    G: Fn(B) -> C,
{
    move |a, b| f(a)(b)
}

pub fn curry<'a, A, B, C, F>(f: F) -> impl FnOnce(A) -> Box<dyn FnOnce(B) -> C + 'a>
where
    A: 'a,
    F: FnOnce(A, B) -> C + 'a,
{
    |a| Box::new(move |b| f(a, b))
}

pub fn curry3<'a, A, B, C, D, F>(
    f: F,
) -> impl FnOnce(A) -> Box<dyn FnOnce(B) -> Box<dyn FnOnce(C) -> D + 'a> + 'a>
where
    A: 'a,
    B: 'a,
    F: FnOnce(A, B, C) -> D + 'a,
{
    |a| Box::new(move |b| Box::new(move |c| f(a, b, c)))
}
