//! Minimal example.
//!
//! This does not do anything except create child structs:
//! - `UserScheme`
//! - `UserDto`
//! - `UserEntity`
//!
//! These structs have the same fields as the parent (`User`) and no derives.
//!
//! See UNPACK_EXAMPLE.md for the unpacked view.

fn main() {
    #[allow(dead_code)]
    #[derive(onionpack::OnionPack)]
    struct User {
        name: String,
        password_hash: String,
        hidden_value: i32,
    }
}
