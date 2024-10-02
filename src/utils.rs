//! Module for rusty things.
//!
//! No [`syn`](https://docs.rs/syn/latest/syn/index.html) or [`proc_macro2`](https://docs.rs/proc-macro2/1.0.86/proc_macro2/index.html) logic.

use std::collections::HashMap;
use syn::Ident;

/// Returns map of vectors with given keys.
/// 
/// Examle:
/// ```
/// let map = generate_acc::<Ident>(&["all", "scheme", "dto", "enttiy"]);
/// println!("{map:?}"); // { "all": [], "scheme": [], "dto": [], "entity": [] }
/// ```
pub(crate) fn generate_acc<T>(keys: &[&str]) -> HashMap<String, Vec<T>> {
    let mut acc = HashMap::with_capacity(keys.len());

    // WARN: Realloc here: `to_string()`.
    keys.iter().for_each(|key| { acc.insert(key.to_string(), Vec::new()); });
    
    acc
}

/// Merges value by `key2` with value by `key1`.
///
/// Example:
/// ```
/// let map = HashMap::from([("all", vec![PartialEq, Eq]), ("dto", vec![Debug])]);
/// let result = merge_vals_to_vec(&map, "all", "dto"); // second += first
/// println!("{result:?}") // [PartialEq, Eq, Debug]
/// ```
///
/// WARN: returns a **new** [`Vec`]. 
fn merge_vals_to_vec(map: &HashMap<String, Vec<Ident>>, key1: &str, key2: &str) -> Vec<Ident> {
    let mut first = map.get(key1).cloned().unwrap_or_else(Vec::new);
    let second = map.get(key2).cloned().unwrap_or_else(Vec::new);

    first.extend(second);

    first
}

/// Unpacks children derives from HashMap.
///
/// Examle:
/// ```
/// let map = HashMap::from([("scheme", vec![PartialEq]), ("dto", vec![Eq]), ("entity", vec![Debug]) ]);
/// let (
///     scheme, // [PartialEq]
///     dto,    // [Eq]
///     entity, // [Debug]
/// ) = unpack_derives(&map);
/// ```
pub(crate) fn unpack_derives(derives: &HashMap<String, Vec<Ident>>) -> (Vec<Ident>, Vec<Ident>, Vec<Ident>) {
    (
        merge_vals_to_vec(derives, "all", "scheme"),
        merge_vals_to_vec(derives, "all", "dto"),
        merge_vals_to_vec(derives, "all", "entity"),
    )
}
