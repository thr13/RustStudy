use rand::Rng;
use std::cmp::Ordering; // 열거형 Ordering 의 배리언트(variant)는 Less, Greater, Equal
use std::io;

fn main() {
    println!("숫자 추측!");

    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=100);

    println!("비밀번호: {secret_number}");
    println!("당신의 추측을 입력하세요.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("라인 읽기에 실패했습니다.");

    /*
    이전 guess 의 값을 새로운 값으로 가리는(shadow) 것이 허용된다. 이걸 섀도잉(shadowing) 이라고 한다. 섀도잉은 어떤 한 타입의 값을 다른 타입으로 변경시킬 수 있다.
    문자열의 parse() 메서드는 문자열을 다른 타입으로 바꿔준다. 변수 뒤의 :(콜론)은 변수의 타입을 명시한 것을 의미한다.
    u32 는 부호가 없는 32bit 정수를 의미한다.(정수형 절대값) 이는 작은 양수를 표현하기 좋다.
    */
    let guess: u32 = guess.trim().parse().expect("숫자 타입으로 부탁드립니다.");

    println!("추측: {guess}");



    /*
    cpm(&비교할값) 메서드는 두 값을 비교하고 비교 가능한 모든 것들을 호출
    match 표현식은 갈래(arm)들로 이뤄져, 하나의 갈래는 하나의 패턴(pattern)과 패턴 일치시 실행할 코드로 이뤄져 있다
    match 에 주어진 값을 갈래의 패턴에 맞는지 순서대로 확인한다.(갈래의 패턴과 일치하지 않을 경우 무시)
     */
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("너무 작습니다!"),
        Ordering::Greater => println!("너무 큽니다."),
        Ordering::Equal => println!("맞췄습니다!"),
    }
}