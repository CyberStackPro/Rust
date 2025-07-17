// - `HashMap<K, V>` is generic over both the key type `K` and the value type `V`.

// - Like `Vec<T>`, `HashMap<K, V>` stores its data on the heap, allowing it to grow or shrink dynamically.

// - Keys must be of a type that can be "hashed" (implement the `Eq` and `Hash` traits). Common types like `String`, `i32`, `bool`, and custom structs (if you implement these traits) work well as keys.

use std::collections::HashMap;

// creating hash map

fn main() {
    // 1. Creating an empty HashMap:
    // Type annotation is often needed initially if no values are inserted immediately.
    let mut scores: HashMap<String, i32> = HashMap::new();

    // 2. Inserting key-value pairs:
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 3. Creating a HashMap from a list of tuples using `collect()`:
    // This is a common way to initialize a map from iterators.
    let teams = vec![String::from("Red Team"), String::from("Green Team")];
    let initial_score = vec![100, 80];

    // `zip` combines two iterators into an iterator of pairs (tuples).
    // `collect()` then turns these pairs into a HashMap.
    let another_team = &teams;
    println!("Team Itrator {:?}", another_team);

    let team_scores: HashMap<_, _> = teams.into_iter().zip(initial_score.into_iter()).collect();
    println!("Team scores: {:?}", team_scores);
}

// accessing a  value using it's key the get() method
fn access_value() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 1. Accessing a value using `get()`:
    // Returns `Some(&value)` if key exists, `None` otherwise.
    let team_name = String::from("Blue");
    let blue_score = scores.get(&team_name); // `get` takes a reference to the key

    match blue_score {
        Some(score) => println!("Blue team score: {}", score), // Output: Blue team score: 10
        None => println!("Blue team not found."),
    }

    let non_existent_team = String::from("Red");
    let red_score = scores.get(&non_existent_team);
    match red_score {
        Some(score) => println!("Red team score: {}", score),
        None => println!("Red team not found."), // Output: Red team not found.
    }
    // You can also use `unwrap()` or `expect()` for quick access, but be careful!
    // let yellow_score = scores.get(&String::from("Yellow")).unwrap();
    // println!("Yellow team score: {}", yellow_score);
    // let missing_score = scores.get(&String::from("Missing")).expect("Team should exist!"); // This would panic!
}

fn iterating_over_hash_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // Iterating over immutable references to key-value pairs:
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // Output order is not guaranteed for HashMaps (it's arbitrary based on hashing).
    // Example output:
    // Yellow: 50
    // Blue: 10

    // Iterating over mutable references to key-value pairs:
    // Allows modifying values.
    for (key, value) in &mut scores {
        println!("Before modification - {}: {}", key, value);
        *value += 10; // Dereference `value` to modify the actual score
        println!("After modification - {}: {}", key, value);
    }
    println!("\nScores after modification: {:?}", scores);

    // Iterating and taking ownership (consuming the HashMap):
    // Use this when you no longer need the HashMap after iteration.
    let owned_map: HashMap<String, String> = [
        (String::from("A"), String::from("1")),
        (String::from("B"), String::from("2")),
    ]
    .iter()
    .cloned()
    .collect(); // .cloned() is needed because iter() gives references

    for (key, value) in owned_map {
        // `key` and `value` are owned `String`s
        println!("Owned item: {}: {}", key, value);
    }
    // println!("{:?}", owned_map); // ERROR: value borrowed here after move
}

// Hash Map Ownership

fn hash_map_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // Ownership of `field_name` and `field_value` moves into `map`

    // println!("{}", field_name); // ERROR: value borrowed here after move
    // println!("{}", field_value); // ERROR: value borrowed here after move

    // If you need to keep the original variables, you must clone them before inserting:
    let k = String::from("Key");
    let v = String::from("Value");
    let mut map2 = HashMap::new();
    map2.insert(k.clone(), v.clone()); // Clone to keep ownership of k and v
    println!("Original k: {}, v: {}", k, v); // This now works
}
