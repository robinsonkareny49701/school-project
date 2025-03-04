// Rust is a statically typed and compiled language
// It's known for its strong type system and memory safety guarantees

use std::collections::HashMap;

fn main() {
    // Create a new hash map
    let mut my_map = HashMap::new();

    // Insert some values into the hash map
    my_map.insert("apple", 3);
    my_map.insert("banana", 4);
    my_map.insert("cherry", 5);

    // Print out the contents of the hash map
    println!("{:?}", my_map);
}
