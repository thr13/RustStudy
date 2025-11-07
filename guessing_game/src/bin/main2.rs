use std::io;
use rand::Rng; // 난수 생성기를 구현한 메서드들을 정의한 트레이트(trait)

fn main() {
    println!("숫자 추측!");

    let mut rng = rand::rng(); // rng() 메서드는 seed 를 OS로 부터 정한 값을 받아와 난수를 생성한다. rand 0.9.x 이상부터 더 이상 thread 와 무관하고, 캐싱, 전역상태가 없어져 호출할때 마다 새로운 인스턴스를 만든다
    let secret_number = rng.random_range(1..=100); // random_range(start..=end) 범위표현식을 가진다

    println!("비밀번호: {secret_number}");
    println!("당신의 추측을 입력하세요.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("라인 읽기에 실패했습니다.");
    println!("추측: {guess}");
}