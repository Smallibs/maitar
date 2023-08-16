# Maitar

When I first read this [blog post](https://varkor.github.io/blog/2019/03/28/idiomatic-monads-in-rust.html)
or read and try to use [this code](https://github.com/cando/Algar) something is going wrong in my point of view.

In fact, in these approaches `Functor` trait is implemented for several data structures but this implies 
the specification of Fun

```rust
impl<'a, F, A> Monad<'a> for Free<'a, F, A>
where ... {
 ...
}
```

