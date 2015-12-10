#![feature(alloc_system)]
extern crate alloc_system;

fn main() {
    let mut y = 0;
    for i in (0..100000) {
        let bar = vec![i as i64; i];
        y += multiply_ints(&bar);
    }

    println!("{}", y);
}

fn sum_floats(v: &Vec<f64>) -> f64 {
    v.iter().fold(0.0, |acc, item| acc + item)
}

#[inline(never)]
fn multiply_ints(v: &Vec<i64>) -> i64 {
    let mut x = 0.0;
    let foo = vec![v.len() as f64; v.len()];
    x += sum_floats(&foo);
    println!("{}", x);

    v.iter().fold(1, |acc, item| acc * item)
}
