mod basket;
mod container;
mod stack;

use basket::Basket;
use container::Container;
use stack::Stack;

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
}

fn main() {
    let mut b1 = Basket::new(String::from("30"));
    let b2 = Basket::new(true);
    let b3 = Basket::new(30);

    let s1 = Stack::new(vec![String::from("23")]);
    let s2 = Stack::new(vec![1, 2, 3]);
    add_string(&mut b1, String::from("hi"));
    println!("{:#?}", b1);
    println!("{:#?}", b2);
    println!("{:#?}", b3);
    println!("{:#?}", s1);
    println!("{:#?}", s2);
}
