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
