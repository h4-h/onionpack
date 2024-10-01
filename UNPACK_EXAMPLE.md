# Unpacked examples

This crate provides 3 macro:
- Derive `OnionPack` - Expands struct into 3 child structs: `<name>Scheme`, `<name>Dto`, `<name>Entity`.
- Attribute `onion_derive` - Appends derives to the child structs.
- Attribute `onion_dist` - Appends child structs with choosen fields.

## Minimal - Simple expanding

If we just slap `OnionPack` into struct crate will generate 3 child structs.

<details>
<summary>Before</summary>

```rust
  #[derive(onionpack::OnionPack)]
  struct User {
    name: String,
    password_hash: String,
    hidden_value: i32,
  }
```

</details>

<details>
<summary>After</summary>

```rust
  struct User {
    name: String,
    password_hash: String,
    hidden_value: i32,
  }

  pub struct UserScheme {
    name: String,
    password_hash: String,
    hidden_value: i32,
  }

  pub struct UserDto {
    name: String,
    password_hash: String,
    hidden_value: i32,
  }

  pub struct UserEntity {
    name: String,
    password_hash: String,
    hidden_value: i32,
  }
```

</details>

### Derives - Expanding with distinct derives

`onion_derive` allows you to add derives to the child structs.

<details>
<summary>Before</summary>

```rust
  #[derive(onionpack::OnionPack)]
  #[onion_derive(
    all(Debug),         // This derives will be on top of all children structs
    dto(PartialEq, Eq), // This derives will be on top of `UserDto`
    entity(Clone)       // This derives will be on top of `UserEntity`
    // There is also `scheme(...)` variant
  )]
  struct User {
    name: String,
    password_hash: String,
    hidden_value: i32,
  }
```

</details>

<details>
<summary>After</summary>

```rust
  struct User {
    name: String,
    password_hash: String,
    hidden_value: i32,
  }

  #[derive(Debug)]                // <--- all(...) + scheme(...)
  pub struct UserScheme {
    name: String,
    password_hash: String,
    hidden_value: i32,
  }

  #[derive(Debug, PartialEq, Eq)] // <--- all(...) + dto(...)
  pub struct UserDto {
    name: String,
    password_hash: String,
    hidden_value: i32,
  }

  #[derive(Debug, Clone)]         // <--- all(...) + entity(...)
  pub struct UserEntity {
    name: String,
    password_hash: String,
    hidden_value: i32,
  }
```

</details>

### Fields - Expanding with distinct fields

`onion_dist` allows you to control which fields will be in the child structs.

It can have 4 values:
- `none` - this field will no appear in the child structs.
- `scheme` - this field will appear only in `<name>Scheme` struct.
- `dto` - this field will appear only in `<name>Dto` struct.
- `entity` - this field will appear only in `<name>Entity` struct.

You can specify multiple values at the same time.

<details>
<summary>Before</summary>

```rust
  #[derive(onionpack::OnionPack)]
  struct User {
    name: String,
    #[onion_dist(dto, entity)]
    password_hash: String,
    #[onion_dist(none)]
    hidden_value: i32,
  }
```

</details>

<details>
<summary>After</summary>

```rust
  struct User {
    name: String,
    password_hash: String,
    hidden_value: i32,
  }

  pub struct UserScheme {
    name: String,
    // no hidden_value because it have `none`
    // no password_hash because it appear only in `dto` and `entity`
  }

  pub struct UserDto {
    name: String,
    password_hash: String,
    // no hidden_value because it have `none`
  }

  pub struct UserEntity {
    name: String,
    password_hash: String,
    // no hidden_value because it have `none`
  }
```

</details>


### Full - full example, derives+fields


<details>
<summary>Before</summary>

```rust
  #[derive(onionpack::OnionPack)]
  #[onion_derive(
    all(Debug),
    dto(PartialEq, Eq),
    entity(Clone)
  )]
  struct User {
    name: String,
    #[onion_dist(dto, entity)]
    password_hash: String,
    #[onion_dist(none)]
    hidden_value: i32,
  }
```

</details>

<details>
<summary>After</summary>

```rust
  struct User {
    name: String,
    password_hash: String,
    hidden_value: i32,
  }

  #[derive(Debug)]
  pub struct UserScheme {
    name: String,
  }

  #[derive(Debug, PartialEq, Eq)]
  pub struct UserDto {
    name: String,
    password_hash: String,
  }

  #[derive(Debug, Clone)]
  pub struct UserEntity {
    name: String,
    password_hash: String,
  }
```

</details>
