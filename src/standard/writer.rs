/*
 * MIT License
 *
 * Copyright (c) 2023 Didier Plaindoux
 */

use crate::core::hkp::HKP;
use std::marker::PhantomData;

pub struct Writer<'a, E: 'a, F: HKP<'a>, A: 'a>(F::T<(A, E)>);

pub struct WriterK<'a, E, F: HKP<'a>>(PhantomData<&'a E>, PhantomData<F>);

impl<'a, E, F: HKP<'a>> HKP<'a> for WriterK<'a, E, F> {
    type T<B: 'a> = Writer<'a, E, F, B>;
}
