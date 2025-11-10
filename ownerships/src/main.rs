/*
fn main() { // 변수 s 는 아직 선언되지 않았기 때문에 이 라인에선 유효하지 않다.
    let s = "hello"; // 이 변수는 선언된 시점부터 현재의 스코프를 벗어날 때까지 유효하다. 이 지점부터 변수 s 는 유효하다.
} // 스코프가 종료되어 변수 s 는 더 이상 유효하지 않다.

fn main() {
    let x = 5; // 5를 x에 바인딩한다. 정수형 값 5은 스택에 푸시된다.
    let y = x; // x값의 복사본을 만들어 y에 바인딩한다. 정수형 값 5은 스택에 푸시된다.

    let s1 = String::from("hello"); // String 타입은 문자열 내용이 들어 있는 메모리를 가르키는 포인터, 문자열 길이(현재 사용하고 있는 메모리를 바이트 단위로 나타낸 것), 메모리 용량 으로 이뤄진다. 이 데이터는 스택에 저장된다. 문자열 내용은 힙 메모리에 저장되어 있다.
    let s2 = s1; // String 타입을 대입시, 스택에 있는 데이터(포인터, 문자열 길이, 용량값)가 복사된다. 그러나 포인터가 가리키는 힙 영역의 데이터는 복사되지 않는다. 문자열 내용이 들어있는 메모리를 가르키는 포인터가 복사된다.
    // 러스트는 메모리 안정성을 위해, 복사한 s1 은 더이상 유효하지 않다고 판단한다. 그리고 Rust 에서는 얕은 복사 개념과 달리, 기존 변수를 무효화하기 때문에 얕은 복사가 아닌 이동(move)되었다 라고 표현한다.
}

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("{}의 길이는 {}이다.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
*/
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
