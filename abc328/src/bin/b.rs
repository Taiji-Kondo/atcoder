use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [u32; n],
    }

    let mut answer: u32 = 0;
    for i in 1..=n {
        for j in 1..=d[i - 1] {
            let input = i.to_string() + &j.to_string();
            if is_rep_digit(input.parse().unwrap()) {
                answer += 1
            }
        }
    }
    println!("{}", answer);
}

fn is_rep_digit(num: u32) -> bool {
    let mut num = num;
    let last_digit = num % 10;
    while num > 0 {
        if num % 10 != last_digit {
            return false;
        }
        num /= 10;
    }
    true
}