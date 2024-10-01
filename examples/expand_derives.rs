//! Here we add derives to the child structs.
//!
//! This appends each child struct with the distinct derives.
//!
//! UserScheme gets derives from `onion_derive.all` + `onion_derive.scheme`.
//! UserDto gets derives from `onion_derive.all` + `onion_derive.dto`.
//! UserEntity gets derives from `onion_derive.all` + `onion_derive.entity`.
//!
//! See UNPACK_EXAMPLE.md for unpacked view.

fn main() {
    #[allow(dead_code)]
    #[derive(onionpack::OnionPack)]
    #[onion_derive(
        all(Debug),
        dto(PartialEq, Eq),
        entity(Clone)
    )]
    struct User {
        name: String,
        password_hash: String,
        hidden_value: i32,
    }
}
