fn main() {
    let number = 3;

    if number < 5 {
        println!("조건이 참");
    } else if number == 3 {
        println!("else if 조건식 실행");
    } else {
        println!("조건이 거짓");
    }

    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("num 값: {num}");
}
