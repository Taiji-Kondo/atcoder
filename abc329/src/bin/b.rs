use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut first: usize = 0;
    for i in 0..n {
        if a[i] > first { first = a[i] }
    }

    let mut second: usize = 0;
    for i in 0..n {
        if second == 0 && a[i] < first {
            second = a[i]
        } else if a[i] < first && a[i] > second {
            second = a[i]
        }
    }

    println!("{}", second);
}

