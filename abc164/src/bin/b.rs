use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(mut a: i32, b: i32, mut c: i32, d: i32);
    loop {
        c -= b;
        if c <= 0 {
            println!("Yes");
            return;
        }
        a -= d;
        if a <= 0 {
            println!("No");
            return;
        }
    }
}
