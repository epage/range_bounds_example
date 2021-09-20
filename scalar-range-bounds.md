- Feature Name: Scalar Range Bounds
- Start Date: 2021-09-20
- RFC PR: [rust-lang/rfcs#3119](https://github.com/rust-lang/rfcs/pull/3119)
- Rust Issue: [rust-lang/rust#88867](https://github.com/rust-lang/rust/issues/88867)

# Summary
[summary]: #summary

Add an `impl RangeBounds<T> for T: Ord` to allow writing code that can be
generic not just over every range type but also integers.

See also
- [nom's current API](examples/current)
- [A naiive, but broken, API](examples/broken-nom-proposal)
- [nom's proposed API](examples/nom-proposal)
- [This RFC's API](examples/rust-proposal)
- [Interaction of nom and this RFC if both go forward](examples/combined)

# Motivation
[motivation]: #motivation

`nom` repeats a lot of functions for various cases.  For example:
- `nom::multi::many0` for parsing `0..` elements
- `nom::multi::many1` for parsing `1..` elements
- `nom::multi::many_m_n` for parsing `m..=n` elements
  - `n` being inclusive is also unclear without reading through the documentation and examples

There is a [proposal](https://github.com/Geal/nom/issues/1393) to simplify
variants down to a single variant.  See an example
[before](examples/current/main.rs) and [after](examples/nom-proposal/main.rs).

`clap` has a similar problem when describing how many values are associated
with an argument and is similarly looking at a [proposal to
consolidate](https://github.com/clap-rs/clap/issues/2688)
- `takes_value`
- `multiple_values`
- `min_values`
- `max_values`

The interaction of these is unclear without reading the source.

While each crate can re-discover `nom`s proposed
[`IntoRangeBounds`](examples/nom-proposa/fake_nom.rs), it requires users to
implement their `IntoRangeBounds` to avoid E0119
(`upstream crates may add a new impl of trait`).  Addressing this in `std`
would lower the barrier for people to generically support ranges which would
make them more likely to do so.

# Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

`RangeBounds` is implemented by Rust's built-in range types, produced
by range syntax like `..`, `a..`, `..b`, `..=c`, `d..e`, `f..=g`, or `h`,
allowing developers to generically operate on any range type.

Examples

Instead of implementing
```rust
fn many0() { ... }
fn many1() { ... }
fn many_m_n(m: usize, n: usize) { ... }
...
many0();
many_m_n(10, 10);
many_m_n(0, 10);
many_m_n(10, usize::MAX);
```

You can write:
```rust
fn many(range: impl RangeBounds<usize>) { ... }
...
many(..);
many(10);
many(0..=10);
many(10..);
```

# Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

Add a new impl for `RangeBounds`:
```rust
impl<T> RangeBounds<T> for T
where
    T: std::cmp::Ord,
{
    fn start_bound(&self) -> Bound<&T> {
        Included(&self)
    }
    fn end_bound(&self) -> Bound<&T> {
        Included(&self)
    }
}
```

Allowing any `Ord` type to be treated as a range.

# Drawbacks
[drawbacks]: #drawbacks

- If any crates are doing what nom is proposing, they will break:
```
error[E0277]: the trait bound `i32: fake_std::ops::RangeBounds<usize>` is not satisfied
  --> examples/combined/main.rs:5:5
   |
5  |     fake_nom::many(10);
   |     ^^^^^^^^^^^^^^ the trait `fake_std::ops::RangeBounds<usize>` is not implemented for `i32`
   | 
  ::: examples/combined/fake_nom.rs:26:8
   |
26 | pub fn many<G, H>(range: G)
   |        ---- required by a bound in this
...
29 |     H: RangeBounds<usize>,
   |        ------------------ required by this bound in `many`
```
- Developers can't provide custom `impl`s for their `Ord` types
- We might not strike the right balance for what types we `impl` for

# Rationale and alternatives
[rationale-and-alternatives]: #rationale-and-alternatives

Alternatives:
- Leave this to crate authors
  - If `nom` is the first to do this and merges their approach, we'll break them if we do this later
  - This adds friction for using ranges in APIs
- `impl RangeBounds` for each integer type
  - Requires crate authors to remember to do this, or else their users won't be able to leverage this
- `impl RangeBounds for T`
  - Conflicts with impl for different range types
  - Clutters docs for unrelated types
  - Developers might unintentionally make too magical / permissive of APIs
- `impl RangeBounds` for `Step`
  - `Step` is required for iterating on a range, so indicative of range-related type
  - `Step` is nightly-only, not allowing crate-authors to opt-in
  - Mapping `N` to `N..=N` assumes `N` is `Eq` but `Step` doesn't guarantee that
  - Might over constrain, there might be types that work with `RangeBounds` but not `Step`
- `impl RangeBounds` for `PartialOrd`
  - Mapping `N` to `N..=N` assumes full equality (`Eq`) rather than partial
- `impl RangeBounds` for `Eq`
  - `Eq` makes sense for `N` for not as a range-type as whole
- Use `From`
  - For each range type, add `impl<'r, T> From<&'r Range> for (Bound<&'r T>, Bound<&'r T>)`
  - For each range type, add `impl<'r, usize> From<&'r usize> for (Bound<&'r usize>, Bound<&'r usize>)`
  - Seems overkill since `trait RangeBounds` acts very similar to a
    `IntoBounds` (with `(Bound<&T>, Bound(&T>)` being the universal concrete
    type with `&` since the range API doesn't even assume `Clone`).

# Prior art
[prior-art]: #prior-art

# Unresolved questions
[unresolved-questions]: #unresolved-questions


# Future possibilities
[future-possibilities]: #future-possibilities

- Add `RangeBounds::to_bounds(&self) -> (Bound<T>, Bound<T>)`
  - As a convenience for storing generic ranges that can be operated on
  - To encourage minimizing code bloat by having wrapper generic functions call
    `to_bounds` that pass `(Bound<T>, Bound<T>)` to their inner implementation.
