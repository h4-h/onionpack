# OnionPack

Crate that will unpack your struct for layered architecture.

## Usage

```rust
// This:

#[derive(onionpack::OnionPack)]
#[onion_derive(
  all(Debug),          // Describes what derives will have all structs
  dto(PartialEq, Eq)   // Describes what derives will have `UserDto`
  // Can be appended wtith `scheme` and `entity` if you need derives for them.
)]
struct User {
  name: String,              // No `onion_dist` mean that this field will appear in all structs.
  #[onion_dist(dto, entity)] // Describes where field will appear: `scheme` and/or `dto` and/or `entity`.
  password_hash: String,
  #[onion_dist(none)]        // Doesn't appear in child structs.
  hidden_value: i32,
}

// Generates this:

#[derive(Debug)]
pub struct UserScheme {
  name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct UserDto {
  name: String,
  password_hash: String,
}

#[derive(Debug)]
pub struct UserEntity {
  name: String,
  password_hash: String,
}
```

See [`UNPACK_EXAMPLE.md`](./UNPACK_EXAMPLE.md) or [`examples`](./examples/) for more examples.

## Contributing

Wanna contribute?
- Are you a tech person? – You can find the `TODO`-list in [`src/lib.rs`](./src/lib.rs).
- Are you an English speaker? – You can correct errors in any sentence in [`README.md`](./README.md), [`UNPACK_EXAMPLE.md`](./UNPACK_EXAMPLE.md), etc.

License: [MIT](./LICENSE).
