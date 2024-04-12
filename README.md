# strongly::typed

[WIP] A proc macro to create strongly-typed primitives.

## Usage

Add the `#[strongly::typed(<primitive>)]` attribute to your unit struct to turn
it into a strongly-typed primitive. Supports all integers and floats, plus
`bool` and `char`.

```rust
#[strongly::typed(u8)]
pub struct SpecialInt;
```

The attribute will also add all nine possible default derives (`Copy`, `Clone`,
`Default`, etc.) and set `#[repr(transparent)]`.

## Features

* Default: `std`
* `std`: Doesn't do anything at the moment.
* `serde`: Generate implementations of `Serialize` and `Deserialize` to and from
  the representation of the primitive.

## Caveats

* It's not possible to loop over strongly-typed integer ranges because the
  `Step` trait is unstable. Instead, the macro generates helper methods to
  create (strongly-typed) iterators.
* Strongly-typed `bool`s cannot be directly used as `if` condition expressions
  nor can they be directly used in `&&` and `||` operations as these are not
  implementable. (TODO: Offer `Deref<Target=bool>` via feature.)
