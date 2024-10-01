mod onion;
mod utils;

// TODO:
//   - Give users ability to choose an access modifier for child structs (`pub`/`pub(crate)`).
//   - Auto impl of `From` trait for `scheme` <-> `dto` <-> `entity`.
//   - Remove realloc on Vec resize in utils.
//   - Add error handling in onion::unpack_fields.
//   - Remove field cloning in onion::unpack_fields.
//   - Write tests :)

#[proc_macro_derive(OnionPack, attributes(onion_derive, onion_dist))]
pub fn onionpack_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    onion::onionpack_impl(&syn::parse_macro_input!(input as syn::DeriveInput)).into()
}
