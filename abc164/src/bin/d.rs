use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {in_s: String};
    let mut nums = HashMap::new();
    let mut val = 0;
    let mut mul = 1;

    nums.insert(0, 1);

    for c in in_s.as_str().chars().rev() {
        let x: usize = c.to_string().parse().unwrap();

        val = (val + mul * x) % 2019;
        mul = (mul * 10) % 2019;

        if !nums.contains_key(&val) {
            nums.insert(val, 0);
        }

        nums.insert(val, nums[&val] + 1);
    }

    let mut count = 0;

    for (_, v) in &nums {
        if *v < 2 {
            continue;
        }

        count += *v * (*v - 1) / 2;
    }

    println!("{}", count);
}
