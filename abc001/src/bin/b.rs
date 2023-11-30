use proconio::input;

fn main() {
    input! {
        m: f32
    }

    let k = m / 1000.0;
    if m < 100.0 {
        println!("00");
    } else if m >= 100.0 && m <= 5000.0 {
        println!("{:02}", k * 10.0);
    } else if m >= 6000.0 && m <= 30000.0 {
        println!("{}", k + 50.0);
    } else if m >= 35000.0 && m <= 70000.0 {
        println!("{}", ((k - 30.0) / 5.0) + 80.0);
    } else {
        println!("89");
    }
}
