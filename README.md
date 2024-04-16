# `#[strongly::typed]`

A proc macro to create strongly-typed primitives.

> [!CAUTION]
> Work in progress. Do not use yet. More details to follow.

## Usage

Add the `#[strongly::typed]` attribute to your newtype struct to turn
it into a strongly-typed primitive. Supports all integers and floats, plus
`bool` and `char`.

```rust
#[strongly::typed(serde)]
pub struct SpecialInt(u8);
```

The attribute will also add all nine possible default derives (`Copy`, `Clone`,
`Default`, etc.) and set `#[repr(transparent)]`.

## Parameters

* __convert__: Generate implemetations of `From`/`Into` between inner and outer
  types. Also add implementation of `Borrow` of all inner primitives except
  floats. Provide `const` helper method to access inner primitive.
* __deref__: Generate implementations of `Deref` and `DerefMut`.
* __serde__: Generate implementations of `Serialize` and `Deserialize` to and from
  the representation of the primitive.

## Features

* Default: __std__
* __std__: Doesn't do anything at the moment. None of the generated code requires
  `std`.

## Caveats

* It's not possible to loop over strongly-typed integer ranges because the
  `Step` trait is unstable. Instead, the macro generates helper methods to
  create (strongly-typed) iterators.
* Strongly-typed `bool`s cannot be directly used as `if` condition expressions
  nor can they be directly used in `&&` and `||` operations as these are not
  implementable. (TODO: Offer `Deref<Target=bool>` via feature.)
