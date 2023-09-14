# Maitar

Maitar is a rust library providing some functional programming constructions 
like:
- Functor,
- Applicative;
- Bind and
- Monad

Some incarnations are available like:
- Identity,
- Option,
- Result,
- Either and
- Vector

# Philosophy

In Rust, we can specify abstract types in a trait, i.e. Higher-Kinded Polymorphism
(HKP for short). Therefore, such types are inhabited by trait implementations.

## HKP

We propose a simple basic trait `HKP` where an abstract type `T` is defined.
An important remark about this type is its capability to accept a parametric type.
This is important if we want to propose constructions inspired by category theory.

```rust
pub trait HKP {
    type T<A>;
}
```

## Functor specification

The functional approach has been chosen to prohibit the use of `self`.
There is no specific relationship between the structure that will implement
the trait and the implementation of the trait.



```rust
use crate::core::hkp::HKP;

pub trait Functor: HKP {
    fn map<A, B, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
        where
            MAP: Fn(A) -> B;
}
```

### Infix trait

```rust
pub mod infix {
    use crate::specs::functor::Functor as Api;

    pub trait Functor<A> {
        type This: Api;
        type T<B>: Functor<B>;

        fn from_hkp<B>(a: <Self::This as HKP>::T<B>) -> Self::T<B>;

        fn to_hkp(self) -> <Self::This as HKP>::T<A>;

        fn map<B, MAP>(self, f: MAP) -> Self::T<B>
            where
                MAP: Fn(A) -> B,
                Self: Sized,
        {
            Self::from_hkp(Self::This::map(f, self.to_hkp()))
        }
    }
}
```

### Functor usage

```rust
fn generalized_increment<F: Functor>(ma: F::T<i32>) -> F::T<i32> {
    F::map(|a| a + 1, ma)
}
```

## Functor implementation

```rust
pub struct OptionK;

impl HKP for OptionK {
    type T<A> = Option<A>;
}

impl Functor for OptionK {
    fn map<A, B, MAP>(f: MAP, ma: Self::T<A>) -> Self::T<B>
        where
            MAP: Fn(A) -> B,
    {
        ma.map(f)
    }
}
```

### Infix implementation

```rust
impl<A> Functor<A> for Option<A> {
    type This = OptionK;
    type T<B> = Option<B>;

    fn from_hkp<B>(a: <Self::This as HKP>::T<B>) -> Self::T<B> {
        a
    }

    fn to_hkp(self) -> <Self::This as HKP>::T<A> {
        self
    }
}
```

Note: In reality, such material is done once per implementation thanks to the `Transform` trait.

# Why Maitar?

See [Maitar](https://www.elfdict.com/w/maitar?include_old=1) definition for more information. Composition is the main
idea behind this word.

# Other approaches and propositions

- [Idiomatic monads in Rust](https://varkor.github.io/blog/2019/03/28/idiomatic-monads-in-rust.html)
- [Algar project](https://github.com/cando/Algar)

# License 

```
MIT License

Copyright (c) 2023 Didier Plaindoux

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
