use proconio::input;

fn main() {
    input! {
        s: String,
    }

    for i in 0..s.len() {
        print!("{} ", &s[i..i + 1]);
    }
}
