use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    // NOTE: 別の文字が削除されてしまう
    // let mut answer: Vec<u32> = vec![];
    // let mut count: u32 = 0;
    //
    // for i in 0..n {
    //     if i == 0 {
    //         count += 1;
    //         answer.push(count + 1);
    //     } else if i == 0 || s[i..i+1] != s[i-1..i] {
    //         answer.push(count + 1);
    //         count = 0;
    //     } else {
    //         count += 1;
    //         answer.push(count + 1);
    //     }
    // }
    //
    // // 並び替え
    // answer.sort();
    // // 重複削除
    // answer.dedup();

    // O(n**2)以上でTLEしてしまう
    // let mut answer: Vec<String> = vec![];
    //
    // for i in 0..n {
    //     let mut word: String = (&s[i..i+1]).to_string();
    //     answer.push(word.clone());
    //     for j in i + 1..n {
    //         if (&s[i..i+1]).to_string() != &s[j..j+1] {
    //             break;
    //         }
    //         let new_word: String = format!("{}{}", word, (&s[j..j+1]).to_string());
    //         word = new_word.clone();
    //         answer.push(new_word);
    //     }
    // }
    //
    // // 並び替え
    // answer.sort();
    // // 重複削除
    // answer.dedup();

    println!("{:?}", answer.len());
}
