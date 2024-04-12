# strongly

[WIP] A proc macro to create strongly-typed primitives.

## Caveats

* It's not possible to loop over strongly-typed integer ranges
  because the `Step` trait is unstable. Instead, the macro generates
  helper methods to create (strongly-typed) iterators.
* Strongly-typed `bool`s cannot be directly used as `if` condition expressions
  nor can they be directly used in `&&` and `||` operations as these are not
  implementable. (TODO: Offer `Deref<Target=bool>` via feature.)
