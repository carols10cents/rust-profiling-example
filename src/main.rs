#![feature(alloc_system)]
extern crate alloc_system;

fn main() {
    let mut x = 0.0;
    for i in (0..150_000) {
        let foo = vec![i as f64; i];
        x += sum_floats(&foo);
    }

    let mut y = 0;
    for i in (0..150_000) {
        let foo = vec![i as i64; i];
        y += multiply_ints(&foo);
    }

    println!("{}, {}", x, y);
}

fn sum_floats(v: &Vec<f64>) -> f64 {
    v.iter().fold(0.0, |acc, item| acc + item)
}

fn multiply_ints(v: &Vec<i64>) -> i64 {
    v.iter().fold(1, |acc, item| acc * item)
}
