#![allow(unused)]

use std::collections::btree_map::IterMut;
#[derive(Debug)]
struct SupermarketItem {
    name: String,
    price: f64,
}
#[derive(Debug)]
struct ShoppingCart {
    items: Vec<SupermarketItem>,
}

impl ShoppingCart {
    fn traverse_items<F>(&mut self, mut operation: F)
    where
        F: FnMut(&mut SupermarketItem),
    {
        for item in &mut self.items {
            operation(item);
        }
    }

    fn checkout<F>(self, operation: F)
    where
        F: FnOnce(Self),
    {
        operation(self);
    }
}

fn main() {
    let mut shopping_cart = ShoppingCart {
        items: vec![
            SupermarketItem {
                name: "Apple".to_string(),
                price: 1.0,
            },
            SupermarketItem {
                name: "Banana".to_string(),
                price: 0.5,
            },
            SupermarketItem {
                name: "Orange".to_string(),
                price: 1.5,
            },
        ],
    };

    let adjust_price = |item: &mut SupermarketItem| {
        item.price *= 0.85;
    };

    let rename_name = |item: &mut SupermarketItem| {
        item.name = item.name.to_uppercase();
    };
    shopping_cart.traverse_items(adjust_price);
    shopping_cart.traverse_items(rename_name);

    let mut total_price = 0.0;
    let checkout_action = |mut cart: ShoppingCart| {
        println!("{cart:?}");
        cart.traverse_items(|item| {
            total_price += item.price;
        });
    };
    shopping_cart.checkout(checkout_action);
    println!("Total price: {:.2}", total_price);
}
