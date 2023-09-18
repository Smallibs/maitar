/*
 * MIT License
 *
 * Copyright (c) 2023 Didier Plaindoux
 */

pub fn uncurry<'a, A, B, C, CURRY>(f: CURRY) -> Box<dyn Fn(A, B) -> C + 'a>
where
    CURRY: Fn(A) -> Box<dyn Fn(B) -> C> + 'a,
{
    Box::new(move |a, b| f(a)(b))
}

pub fn curry<'a, A, B, C, UNCURRY>(
    f: UNCURRY,
) -> Box<dyn FnOnce(A) -> Box<dyn FnOnce(B) -> C + 'a> + 'a>
where
    A: 'a,
    UNCURRY: FnOnce(A, B) -> C + 'a,
{
    Box::new(move |a| Box::new(move |b| f(a, b)))
}
