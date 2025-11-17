use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("숫자 추측!");

    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=100);

    println!("비밀번호: {secret_number}");

    loop {
        println!("당신의 추측을 입력하세요.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("라인 읽기에 실패했습니다.");

        // match 표현식을 사용해 잘못된 결과값을 처리할 수 있다. Ok() 는 성공적인 수행, 수행 실패시 에러 정보를 담은 Err()를 반환한다. 밑줄(_)는 모든 값에 매칭 될 수 있는 포괄(catch-all)값 이다.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("추측: {guess}");

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