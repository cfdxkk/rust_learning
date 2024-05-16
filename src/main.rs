use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let target_number = rand::thread_rng().gen_range(1..101);

    println!("target number is {}", target_number);

    println!("猜数（1 - 100 之间）");

    loop {
        println!("请输入一个数并回车：");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取输入内容出错！");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("您输入的可能不是一个 u32 整数！");
                continue;
            },
        };
    

        match guess.cmp(&target_number) {
            Ordering::Less => println!("你猜的数字 {} 太小了！", guess),
            Ordering::Greater => println!("你猜的数字 {} 太大了！", guess),
            Ordering::Equal => {
                println!("你猜对了！{} 是正确的数字！", guess);
                break;
            },
        }
    }
}
