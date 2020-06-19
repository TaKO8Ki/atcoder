use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(n: usize, k: usize, d: [i32; k],);
    let mut count: usize = 0;
    dfs(n, 0, &mut count, 0, 0, 0);
    println!("{}", count);
}

fn dfs(n: usize, a: usize, count: &mut usize, x: usize, y: usize, z: usize) {
    if a <= n && x >= 1 && y >= 1 && z >= 1 {
        *count += 1
    }
    if a >= n {
        return;
    }
    dfs(n, a * 10 + 3, count, x + 1, y, z);
    dfs(n, a * 10 + 5, count, x, y + 1, z);
    dfs(n, a * 10 + 7, count, x, y, z + 1);
}
