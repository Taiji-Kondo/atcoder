use proconio::input;

fn main() {
    input! {
        n: [usize;2],
        s: String,
        lr: [[usize;2];n[1]],
    }

    for i in 0..n[1] {
        let l: usize = lr[i][0];
        let r: usize = lr[i][1];
        let split_s = &s[l-1..r];
        let mut count: u32 = 0;

        for i in 0..split_s.chars().count() {
            if i + 1 < split_s.chars().count() && split_s[i..i+1] == split_s[i+1..i+2] {
                count += 1;
            }
        }
        println!("{}", count);
    }
}
