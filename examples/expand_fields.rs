//! Here we decide what fields will be added to the child structs.
//!
//! `onion_dist` have four types: `scheme`, `dto`, `entity` and `none`.
//!
//! If there is no `onion_dist` attribute, the field appended to all child structs.
//! If `onion_dist` have value of `none`, the field is not append to any child structs.
//!
//! See UNPACK_EXAMPLE.md for unpacked view.

fn main() {
    #[allow(dead_code)]
    #[derive(onionpack::OnionPack)]
    struct User {
        name: String,
        #[onion_dist(dto, entity)]
        password_hash: String,
        #[onion_dist(none)]
        hidden_value: i32,
    }
}
