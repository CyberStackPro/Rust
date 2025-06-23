fn change_string(some_str: &mut String) {
    some_str.push_str(", world");
}

fn main() {
    let mut s = String::from("Hello");

    let r1 = &mut s;

    println!("r1: {}, ", r1);

    change_string(r1);

    // let r2 = &mut s;

    println!("After change, r1: {}", r1);

    let r2 = &mut s;
    r2.push_str("!");
    println!("r2: {}", r2);
}
