use proconio::{fastout, input};
use std::collections::HashSet;

fn main() {
    input!(n: usize, v: [String; n]);
    let uniq: HashSet<String> = v.into_iter().collect();
    println!("{}", uniq.len());
}
