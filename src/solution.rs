/// In Rust, the `type` keyword is used to define an alias for an existing type
/// If you want, you can learn more about it [here](https://doc.rust-lang.org/std/keyword.type.html)
type Inventory<'a> = Vec<(i32, &'a str)>;

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
    for index_i in 0..new_inv.len() {
        let (new_value, name) = new_inv[index_i];
        let mut count = 0;
        for index_j in 0..cur_inv.len() {
            let (cur_value, cur_name) = cur_inv[index_j];
            if cur_name == name {
                let new_val = cur_value + new_value;
                cur_inv[index_j] = (new_val, name);
            }
            if cur_name != name {
                count += 1;
            }
            if count == cur_inv.len() {
                cur_inv.push(new_inv[index_i]);
            }
        }
    }
    cur_inv.sort_by_key(|key| key.1);
    return cur_inv;
}
