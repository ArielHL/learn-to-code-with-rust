#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt::{Debug, Display, Formatter, Result};

trait Drinkable {
    fn consume(&mut self);
    fn get_data(&self) -> String;
    fn stats(&self) {
        println!("{}", self.get_data())
    }
}

#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}
// Coffee Struct /////////////////////////////////////////////////////////////////////
struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

impl<T> Coffee<T> {
    fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self { kind, milk, ounces }
    }
}

impl<T: Debug> Debug for Coffee<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("Coffee")
            .field("kind", &self.kind)
            .field("milk", &self.milk)
            .field("ounces", &self.ounces)
            .finish()
    }
}

impl<T: Display> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces = 0;
    }

    fn get_data(&self) -> String {
        format!("a Delicious {} ounce {}", self.ounces, self.kind)
    }
}

// Soda Struct /////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percent: u32,
}

impl Soda {
    fn new(calories: u32, price: f64, flavor: String) -> Self {
        Self {
            calories,
            price,
            flavor,
            percent: 100,
        }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percent = 0;
    }

    fn get_data(&self) -> String {
        format!("A Flavor {}, calories {}", self.flavor, self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(
            formatter,
            "A {} calorie {} soda that is {} percent full",
            self.calories, self.flavor, self.percent
        )
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories,
            price: self.price,
            flavor: self.flavor.clone(),
            percent: self.percent,
        }
    }
}

impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Soda {}

fn main() {
    let mut latter = Coffee::new("Latte", Milk::Whole, 12);
    println!("{:?}", latter);
    latter.consume();
    println!("{:?}", latter);

    let capuchino = Coffee::new("Capuchino", Milk::Almond, 16);
    println!("{}",capuchino.get_data());


    let pepsi = Soda::new(150, 1.50, "Cherry".to_string());
    println!("{}", pepsi);

    let mut coke = pepsi.clone();
    println!("{}", coke == pepsi);
    coke.consume();
    println!("{}", coke);

}   
