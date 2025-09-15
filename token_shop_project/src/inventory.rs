use crate::{auth::User, error::InventoryError};

#[derive(Clone)]
pub struct Item {
    id: i32,
    name: String,
    price: i32,
    // stock: Option<i32>,
    stock: i32,
}

fn add_item(item: Item, db: &mut Vec<Item>) -> Result<Item, InventoryError> {
    let new_item = Item {
        id: (db.len() + 1) as i32,
        name: item.name,
        price: item.price,
        stock: item.stock,
    };

    if db.iter().any(|i| i.name == new_item.name) {
        return Err(InventoryError::DuplicateItem);
    };
    if new_item.price <= 0 {
        return Err(InventoryError::InvalidPrice);
    };

    // if let Some(s) = new_item.stock {
    //     if s < 0 {
    //         return Err(InventoryError::OutOfStock);
    //     }
    // }
    if new_item.stock < 0 {
        return Err(InventoryError::InvalidStock);
    }

    db.push(new_item.clone());
    Ok(new_item)
}

fn buy_item(
    user: &mut User,
    item_id: i32,
    qty: i32,
    db: &mut Vec<Item>,
) -> Result<(), InventoryError> {
    // let item = db.iter()
    if db.iter().find(|item| item.id == item_id).is_none() {
        return Err(InventoryError::ItemNotFound);
    }

    if qty < 0 {
        return Err(InventoryError::InvalidQuantity);
    }

    if db.iter().find(|i| i.stock < qty).is_none() {
        return Err(InventoryError::OutOfStock);
    };

    let item = db.iter_mut().find(|item| item.id == item_id).unwrap();
    let total_cost = item.price * qty;
    if user.tokens < total_cost {
        return Err(InventoryError::NotEnoughTokens);
    }
    item.stock -= qty;
    user.tokens -= total_cost;
    Ok(())
}

fn remove_item() {}

fn list_item() {}
