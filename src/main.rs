use std::io::{Write};

fn main() {
    println!("this application is pig_latin");
    loop {
        // 標準入力から読み込む
        print!("$ ");
        std::io::stdout().flush().unwrap();
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        s = s.trim().parse().ok().unwrap();

        // exit または q が入力されたらループから抜け、プログラムを終了する
        if s == "exit" || s == "q" { break; }

        // let s = String::from("Hello");
        // println!("a input word is {}", s);
        // println!("a first charactor is {}", &s[..1]);
        let result = pig_latin(s);
        println!("result is {}", result);
        /*
        let s = String::from("Okey");
        println!("a input word is {}", s);
        println!("a first charactor is {}", &s[..1]);
        pig_latin(s);
        println!("result is {}", result);
        */
    }
}

// 母音が含まれているか判定する
fn is_vowel(s: &str) -> bool {
    let vowels = ['a', 'i', 'u', 'e', 'o', 'A', 'I', 'U', 'E', 'O'];
    match vowels.iter().find(|x| **x == s.chars().nth(0).unwrap()) {
        Some(_) => true,
        None => false,
    }
}

// ピッグラテンを行い、結果を返す関数
fn pig_latin(s: String) -> String {
    if is_vowel(&s) {
        // println!("is first charactor is vowel");
        let s = format!("{}{}", s, "hay");
        s
    } else {
        // println!("is first charactor is consonant");
        let s = format!("{}{}{}", &s[1..], &s[..1], "ay");
        s
    }
}
