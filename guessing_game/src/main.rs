use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("数当てゲーム！！\n");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("数字を当てて！");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("read_line() エラー");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("数字は 1 ～ 100 のどれかです")
        }

        println!("予想した数字 : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さい！"),
            Ordering::Greater => println!("大きい！"),
            Ordering::Equal => {
                println!("正解！");
                break;
            }
        }
    }
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("数字が 1 ～ 100 の間でなかった {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}