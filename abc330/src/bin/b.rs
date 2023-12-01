use proconio::input;


fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        a: [usize; n],
    }

    for i in 0..n {
        if a[i] < l {
            print!("{} ", l);
        } else if a[i] > r {
            print!("{} ", r);
        } else {
            print!("{} ", a[i]);
        }
    }
}
