use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("숫자 추측!");

    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=100);

    println!("비밀번호: {secret_number}");

    // 반복문 실행 loop {}
    loop {
        println!("당신의 추측을 입력하세요.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("라인 읽기에 실패했습니다.");

        let guess: u32 = guess.trim().parse().expect("숫자 타입으로 부탁드립니다.");

        println!("추측: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("너무 작습니다!"),
            Ordering::Greater => println!("너무 큽니다."),
            Ordering::Equal => {
                println!("맞췄습니다!");
                break; // 반복문 종료
            }
        }
    }
}