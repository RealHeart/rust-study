use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("这是一个猜数游戏~");

    // 生成随机数 范围 (1-100)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Debug
    println!("secret_number: {:?}", secret_number);

    loop {
        println!("请输入一个数字:");

        // 获取用户输入
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 将输入解析为数字
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("输入错误，请输入一个数字");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("猜小了~"),
            Ordering::Greater => println!("猜大了~"),
            Ordering::Equal => {
                println!("你猜对了~ 是 {}", guess);
                break;
            }
        }
    }
}
