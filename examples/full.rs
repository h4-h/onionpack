//! Full example.
//!
//! See other examples and UNPACK_EXAMPLE.md for more information.

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
        #[onion_dist(dto, entity)]
        password_hash: String,
        #[onion_dist(none)]
        hidden_value: i32,
    }
}
