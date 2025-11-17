use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=100);

    loop {
        println!("당신의 추측을 입력하세요");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("라인 읽기에 실패하였습니다");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("당신의 추리: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("너무 작습니다!"),
            Ordering::Greater => println!("너무 큽니다."),
            Ordering::Equal => {
                println!("맞췄습니다!");
                break;
            }
        }
    }
}