use proconio::{fastout, input};
use std::f64::consts::PI;

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }
    let rad = PI * 2.0 * (h / 12.0 + (m / 60.0) / 12.0 - m / 60.0);
    let ans = (a * a + b * b - 2.0 * a * b * rad.cos()).sqrt();
    println!("{}", ans);
}
