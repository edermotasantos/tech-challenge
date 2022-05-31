/// In Rust, the `type` keyword is used to define an alias for an existing type
/// If you want, you can learn more about it [here](https://doc.rust-lang.org/std/keyword.type.html)
type Inventory<'a> = Vec<(i32, &'a str)>;

/// Don't worry about the `'a` syntax, this is a sligtly advanced concept in Rust called Lifetimes
/// but you don't need to deeple understand this in order to complete this challenge.
/// You can learn more about lifetimes [here](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html).
pub fn update_inventory<'a>(cur_inv: Inventory<'a>, new_inv: Inventory<'a>) -> Inventory<'a> {
    todo!()
    /// primeiro commit 
}
