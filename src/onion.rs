//! Module for [`syn`](https://docs.rs/syn/latest/syn/index.html) and [`proc_macro2`](https://docs.rs/proc-macro2/1.0.86/proc_macro2/index.html).

use std::collections::HashMap;
use proc_macro2::{Span, TokenStream, TokenTree};
use syn::{punctuated::Punctuated, spanned::Spanned, Attribute, DataStruct, DeriveInput, Field, Fields, Ident, Meta, Token};
use crate::utils::{generate_acc, merge_vals_to_vec};

/// Returns children structs.
pub(crate) fn onionpack_impl(input: &DeriveInput) -> TokenStream {
    let fields = match input.data {
        syn::Data::Struct(DataStruct { ref fields, .. }) => fields,
        _ => return generate_error(input.ident.span(), "`OnionPack` works only with structs."),
    };

    let scheme_ident = quote::format_ident!("{}Scheme", &input.ident);
    let dto_ident = quote::format_ident!("{}Dto", &input.ident);
    let entity_ident = quote::format_ident!("{}Entity", input.ident);
    
    let hehe = unpack_fields(fields);

    let scheme_fields = hehe.get("scheme").unwrap();
    let dto_fields = hehe.get("dto").unwrap();
    let entity_fields = hehe.get("entity").unwrap();

    let derives = match parse_onion_derive(input) {
        Ok(ok) => ok,
        Err(err) => return err,
    };

    let scheme_derives = merge_vals_to_vec(&derives, "all", "scheme");
    let dto_derives = merge_vals_to_vec(&derives, "all", "dto");
    let entity_derives = merge_vals_to_vec(&derives, "all", "entity");
    
    quote::quote! {
        #[derive( #(#scheme_derives),* )]
        pub struct #scheme_ident {
            #(#scheme_fields),*
        }

        #[derive( #(#dto_derives),* )]
        pub struct #dto_ident {
            #(#dto_fields),*
        }

        #[derive( #(#entity_derives),* )]
        pub struct #entity_ident {
            #(#entity_fields),*
        }
    }
}

/// Iterates over fields and collects them to the distinct children list.
///
/// Example:
/// ```
/// // I'm lazy. So this is rust pseudocode.
/// fields = [
///     Field { name: x, type: i32, attrs: [onion_dist(dto)] }
///     Field { name: y, type: i32, attrs: [] }
///     Field { name: z, type: String, attrs: [onion_dist(none)]}
/// ];
/// let (
///     scheme, // [Field.name = x]
///     dto,    // [Field.name = x, Field.name = y]
///     entity, // [Field.name = x]
/// ) = unpack_fields(&fields);
/// ```
///
/// As you can see without `onion_dist` field distributes to all children structs.
/// And with `onion_dist(none)` it **does not** appear it any children struct.
// FIXME: remove cloning.
fn unpack_fields(fields: &Fields) -> HashMap<String, Vec<Field>> {
    let mut acc = generate_acc(&["scheme", "dto", "entity"]);

    for field in fields {
        // FIXME: safe unwrap, but yk.
        let Some(onion_dist) = find_attribute("onion_dist", &field.attrs) else {
            acc.get_mut("scheme").unwrap().push(field.clone());
            acc.get_mut("dto").unwrap().push(field.clone());
            acc.get_mut("entity").unwrap().push(field.clone());
            continue;
        };

        let Ok(args) = onion_dist.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) else {
            continue; // FIXME: Change return type to the [`Result`] and return error.
        };

        if args.iter().any(|a| a.path().is_ident("none")) {
            continue;
        }

        for arg in args.iter() {
            if let Meta::Path(ref arg) = arg {
                if let Some(arg_path) = arg.get_ident().map(|id| id.to_string()) {
                    let mut new_field = field.clone();
                    new_field.attrs.retain(|a| !a.path().is_ident("onion_dist"));

                    let _ = acc.get_mut(&arg_path).map(|fv| { fv.push(new_field); });
                };

            }
        }
    }

    acc
}

/// Finds and parse `onion_derive()` attribute to the HashMap for simplier work later.
///
/// Example:
/// ```
/// let input = {
///     #[onion_derive(all(Debug), dto(PartialEq, Eq))]
///     struct Nothing;
/// }
/// let result = parse_onion_derive(&input);
/// // result.all = [Debug]
/// // result.scheme = [Debug]
/// // result.dto = [Debug, PartialEq, Eq]
/// // result.entity = [Debug]
/// ```
fn parse_onion_derive(input: &DeriveInput) -> Result<HashMap<String, Vec<Ident>>, TokenStream> {
    let mut derives = generate_acc(&["all", "scheme", "dto", "entity"]);

    let Some(onion_derive) = find_attribute("onion_derive", &input.attrs) else {
        return Ok(derives);
    };
    
    let Ok(onion_derive_parsed) = onion_derive.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) else {
        return Err(Default::default());
    };

    for derive_lists in onion_derive_parsed {
        if let Meta::List(derive_list) = derive_lists {
            let Some(meta_path) = derive_list.path.get_ident().map(|id| id.to_string()) else {
                continue;
            };

            let Some(idents) = derives.get_mut(&meta_path) else {
                return Err(generate_error(derive_list.span(), format!("Unknown field: {meta_path}").as_str()));
            };

            for derive in derive_list.tokens {
                if let TokenTree::Ident(derive) = derive {
                    idents.push(derive);
                }
            }
        }
    }

    Ok(derives)
}

/// Helper [`inline`] function, returns `syn::Error::new(span, message).to_compile_error()`
#[inline]
fn generate_error(span: Span, message: &str) -> TokenStream {
    syn::Error::new(span, message).to_compile_error()
}

/// Helper [`inline`] function, returns `attrs.iter().find(|a| a.path().is_ident(name))`
#[inline]
fn find_attribute<'a>(name: &str, attrs: &'a [Attribute]) -> Option<&'a Attribute> {
    attrs.iter().find(|a| a.path().is_ident(name))
}
