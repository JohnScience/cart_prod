# Cartesian product of iterators

[![Latest Version](https://img.shields.io/crates/v/cart_prod.svg)][`cart_prod`]
[![Downloads](https://img.shields.io/crates/d/cart_prod.svg)][`cart_prod`]
[![Documentation](https://docs.rs/cart_prod/badge.svg)][`cart_prod`/docs]
[![License](https://img.shields.io/crates/l/cart_prod.svg)][`cart_prod`/license]
[![Dependency Status](https://deps.rs/repo/github/JohnScience/cart_prod/status.svg)][`cart_prod`/dep_status]
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/JohnScience/cart_prod/rust.yml)

At the moment of writing, this crate provides only [`Hom2FCartProd`] and [`Hom3FCartProd`] types, which are a two-fold and three-fold Cartesian products of iterators that are "homogenous" in the sense that they produce elements of the same type. Iterators of [`Hom2FCartProd`] allow to iterate over all distinct pairs `[x,y]` that can be formed from elements `x` and `y` of the original iterators. Similarly, [`Hom3FCartProd`] allows iteration over triples `[x,y,z]`. The order of the elements in the pairs is preserved.

## Example

```rust
use cart_prod::specs::Hom2FCartProd;

let it1 = 0..=2;
let it2 = 0..=1;

let mut it = Hom2FCartProd::new(it1, it2);
assert!(it.next() == Some([0, 0]));
assert!(it.next() == Some([0, 1]));
assert!(it.next() == Some([1, 0]));
assert!(it.next() == Some([1, 1]));
assert!(it.next() == Some([2, 0]));
assert!(it.next() == Some([2, 1]));
assert!(it.next() == None);
```

## Features

* Support for `no_std` environments.
* Usage of arrays instead of tuples for the elements of the Cartesian product can simplify and speed up the code in some cases.

## Implementation notes

### No variadic genericity

Ideally, `Hom*FCartProd` should be types-aliases for a partial specializations of `CartProd` (variadic) generic type. However, Rust does not support variadic generics. See <https://github.com/rust-lang/rust/issues/10124>. For forward compatibility, the [`Hom2FCartProd`] and [`Hom3FCartProd`] types are defined in the [`cart_prod::specs`] module.

### Workaround for absence of variadic genericity

It is possible to provide the type definitions as well as implementations (locally) via macros. However, note that at the moment macros cannot evaluate constants, let alone constant expressions. See <https://github.com/rust-lang/rfcs/issues/2279>. Due to that (and scarcity of time), such macros are currently not provided.

### No support for tuple indexing

Unlike in C++ with [`std::get`], In Rust there's no way to index a tuple by a constant expression. Therefore, it's also impossible to iterate over the tuple of iterators. While its possible to take advantage of trait objects, it's still impossible to generically construct iterators due to absence of variadic genericity.

### No top-notch performance guarantees

While the implementations are reasonably efficient, no work was done to benchmark it or to optimize it. While implementation of [`core::iter::Iterator::next`] is sufficient for implementation of the trait, it is not the most efficient way to implement it. Only the default implementation of [`core::iter::Iterator::size_hint`] was overridden.

### No correctness guarantees

The implementation of [`cart_prod`] has only a few simple tests. In case of issues, please [report them][`cart_prod`/issues].

## Alternatives

* [`itertools`] crate provides [`iproduct!`] macro that can be used to create an n-fold Cartesian product of iterators. However, the macro can't check if the iterators are homogenous, so it will always iterate over tuples.

* [`cartesian`] crate provides [`cartesian!`] macro that also can be used to create an n-fold Cartesian product of iterators. However, the macro can't check if the iterators are homogenous, so it will always iterate over tuples. Also, at the moment of writing, the crate has not been maintained for 2+ years.

* [`permutator`] crate provides [`cartesian_product`], [`cartesian_product_cell`], and [`cartesian_product_sync`] functions that can be used to create an n-fold Cartesian product of slices (and not iterators).

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[`cart_prod`]: https://crates.io/crates/cart_prod
[`cart_prod`/docs]: https://docs.rs/cart_prod
[`cart_prod`/license]: https://github.com/JohnScience/cart_prod#license
[`cart_prod`/issues]: https://github.com/JohnScience/cart_prod/issues
[`cart_prod`/dep_status]: https://deps.rs/repo/github/JohnScience/cart_prod
[`Hom2FCartProd`]: https://docs.rs/cart_prod/latest/cart_prod/specs/struct.Hom2FCartProd.html
[`Hom3FCartProd`]: https://docs.rs/cart_prod/latest/cart_prod/specs/struct.Hom3FCartProd.html
[`std::get`]: https://en.cppreference.com/w/cpp/utility/tuple/get
[`cart_prod::specs`]: https://docs.rs/cart_prod/latest/cart_prod/specs/index.html
[`core::iter::Iterator::next`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
[`core::iter::Iterator::size_hint`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.size_hint
[`itertools`]: https://crates.io/crates/itertools
[`iproduct!`]: https://docs.rs/itertools/latest/itertools/macro.iproduct.html
[`cartesian`]: https://crates.io/crates/cartesian
[`cartesian!`]: https://docs.rs/cartesian/0.2.1/cartesian/macro.cartesian.html
[`permutator`]: https://crates.io/crates/permutator
[`cartesian_product`]: https://docs.rs/permutator/latest/permutator/fn.cartesian_product.html
[`cartesian_product_cell`]: https://docs.rs/permutator/latest/permutator/fn.cartesian_product_cell.html
[`cartesian_product_sync`]: https://docs.rs/permutator/latest/permutator/fn.cartesian_product_sync.html
