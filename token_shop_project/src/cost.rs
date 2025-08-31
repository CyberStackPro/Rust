use crate::error::ShopErr;

const ITEM_PRICE: i32 = 5;

pub fn total_cost(qty: i32) -> i32 {
    qty * ITEM_PRICE
}

pub fn remaining_tokens(qty: i32, tokens: i32) -> Result<i32, ShopErr> {
    // tokens - qty
    let cost = total_cost(qty);
    if cost > tokens {
        Err(ShopErr::NotEnoughTokens)
    } else {
        Ok(tokens - cost)
    }
}

fn return_ref<'a>(input: &'a i32) -> &'a i32 {
    input
}

fn main() {
    let result = {
        let x = 5;
        return_ref(&x)
    };
    println!("{}", result);
}

// fn main() {
//     let s = String::from("Hello");
//     let s = takes_owner_ship(s);
//     println!("{}", s);
// }

// fn takes_owner_ship(string: String) -> String {
//     println!("{}", string);
//     string
// }
