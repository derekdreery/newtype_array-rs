This crate has a single macro, `newtype_arrays`, that will create transparent newtypes for arrays,
and implement standard traits for them. It will be redundant when generic cosntants land, in the
mean time it means you can use large arrays on stable rust.

# Examples

```rust
#[macro_use]
extern crate newtype_array;

use std::collections::HashMap;

// Sha385 length
newtype_array!(pub struct Array48(pub 48));
// Sha512 length
newtype_array!(pub struct Array64(pub 64));

// We've got `Clone` and `PartialEq`/`Eq`
let arr1 = Array48([0u8; 48]);
let arr2 = arr1.clone();
assert_eq!(arr1, arr2);

// `Hash` is implemented as well
let mut map = HashMap::new();
map.insert(arr1, "hello");
```
