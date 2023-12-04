use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u32,
        s: [u32; n],
    }

    let mut total: u32 = 0;
    for i in 0..n {
        if s[i] <= x {
            total += s[i]
        }
    }
    println!("{}", total);
}
