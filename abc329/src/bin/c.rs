use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let mut answer: Vec<String> = vec![];

    for i in 0..n {
        let mut word: String = (&s[i..i+1]).to_string();
        answer.push(word.clone());
        for j in i + 1..n {
            if (&s[i..i+1]).to_string() != &s[j..j+1] {
                break;
            }
            let new_word: String = format!("{}{}", word, (&s[j..j+1]).to_string());
            word = new_word.clone();
            answer.push(new_word);
        }
    }

    // 並び替え
    answer.sort();
    // 重複削除
    answer.dedup();

    println!("{:?}", answer.len());
}
