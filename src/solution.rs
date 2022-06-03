/// In Rust, the `type` keyword is used to define an alias for an existing type
/// If you want, you can learn more about it [here](https://doc.rust-lang.org/std/keyword.type.html)
type Inventory<'a> = Vec<(i32, &'a str)>;
use std::collections::HashMap;

/// Don't worry about the `'a` syntax, this is a sligtly advanced concept in Rust called Lifetimes
/// but you don't need to deeple understand this in order to complete this challenge.
/// You can learn more about lifetimes [here](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html).
pub fn update_inventory<'a>(
    mut cur_inv: Inventory<'a>,
    mut new_inv: Inventory<'a>,
) -> Inventory<'a> {
    new_inv.sort_by_key(|key| key.1);
    if cur_inv.len() == 0 {
        return new_inv;
    }
    let mut map = HashMap::new();
    for index_i in 0..cur_inv.len() {
        let (cur_value, cur_name) = cur_inv[index_i];
        map.insert(cur_name, cur_value);
    }
    for index_j in 0..new_inv.len() {
        let (new_value, name) = new_inv[index_j];
        match map.get(name) {
            Some(val) => {
                let new_val = val + new_value;
                let index_element = cur_inv.iter().position(|&x| x == (*val, name)).unwrap();
                cur_inv[index_element] = (new_val, name);
            }
            None => cur_inv.push(new_inv[index_j]),
        }
    }
    cur_inv.sort_by_key(|key| key.1);
    return cur_inv;
}
