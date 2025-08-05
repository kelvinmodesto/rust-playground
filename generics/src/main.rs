use num_traits::{Float, ToPrimitive};

// First version - accept both f32 or both f64
// Second version - accept any type of numbers

fn solve<T: Float, U: Float>(a: T, b: U) -> f64 {
    let a_64 = a.to_f64().unwrap();
    let b_64 = b.to_f64().unwrap();

    (a_64.powi(2) + b_64.powi(2)).sqrt()
}

fn solve2<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_64 = a.to_f64().unwrap();
    let b_64 = b.to_f64().unwrap();

    (a_64.powi(2) + b_64.powi(2)).sqrt()
}

fn main() {
    let a: f64 = 3.0;
    let b: f32 = 4.0;

    println!("{}", solve(a, b));
}
