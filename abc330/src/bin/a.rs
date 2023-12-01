use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        a: [usize; n],
    }

    let mut answer = 0;

    for i in 0..n {
        if a[i] >= l {
            answer = answer + 1;
        }
    }

    println!("{}", answer);
}
