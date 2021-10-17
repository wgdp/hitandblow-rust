use std::io::stdin;
use std::io::stdout;
use std::io::Write;

extern crate regex;

use regex::Regex;

mod ascii;


struct Title {
    text: String
}
fn draw_title(title: &Title) {
   println!("{}", title.text);
}

fn validate (text: String) -> bool {
    let re = Regex::new(r"^\d{4}$").unwrap();
    let new_text = &text.trim();
    re.is_match(new_text)
}

fn update() {
    print!("Hit and blowは、ランダムに生成された4桁の数字を当てるゲームです。\n数字を入力してください＞");
    stdout().flush().unwrap();
    let mut input_num = String::new();
    stdin().read_line(&mut input_num).expect("失敗");
    println!("{}", input_num);
    if !validate(input_num) {
    }

}
fn main() {
    let title = Title {
        text: ascii::TITLE.to_string()
    };
    draw_title(&title);
    
    loop {
        update();
    }
    
}
