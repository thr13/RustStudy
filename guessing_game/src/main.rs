use std::io; // 입출력 표준 라이브러리

// fn(매개변수) {본문} 새로운 함수 선언
fn main() { // main 함수의 프로그램 진입점
    println!("숫자 추측!"); //println!("") 문자열 화면 출력
    println!("당신의 추측을 입력하세요.");

    /*
    let 변수생성(기본은 불변)
    mut 변수의 값을 가변으로 설정(변수의 값을 변경가능하게 만듦)
    = 변수에 값을 바인딩(bind)
    :: 연관 함수 선언, 앞에는 타입 뒤에는 함수명(파라미터)를 붙여서 사용한다
    String 표준 라이브러리에서 제공하는 UTF-8 인코딩 문자열 타입
     */
    let mut guess = String::new();
    
    /*
    io 라이브러리를 가져오지 않았을 경우 std::io::stdin 처럼 작성해한다
    .read_line() 표준 입력 핸들에서 read_line 메서드를 호출한다. 여기서 &mut guess는 변경가능한 값을 가지는 guess 라는 변수를 함수의 인수로 전달한다
    & 앰퍼샌드는 여러번 메모리로 복사하지 않고 접근하기 위한 참조자를 나타낸다
    read_line() 메소드는 하나의 Result 값을 돌려주는데 이는 타입은 열거형(enum, enumeration) 이고 여러 개의 가능한 값 중 하나를 배리언트(variant)라고 부른다
    Result 의 variant 는 Ok 와 Err 가 있다 그리고 Result 인스턴스는 expect() 메서드를 가지는데 이 메서드로 프로그램 작동을 멈추고 인수로 넘긴 메시지를 출력할 수 있다
    만약 expect() 를 호출하지 않은 경우, 컴파일은 되지만 사용되지 않은 Result 경고가 발생한다
     */
    io::stdin()
        .read_line(&mut guess)
        .expect("라인 읽기에 실패했습니다.");

    /*
    println!("{}") 메서드 내부의 중괄호{}는 자리표시자(placeholder)로 사용자가 입력한 값을 담고 있는 변수의 값을 문자열로 출력한다
    어떤 표현식의 결과값을 출력할 때는 빈 중괄호를 사용하고 쉼표로 구분된 표현식을 나열하여 출력할 수 있다
    예)
    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);
    */
    println!("추측: {guess}");
}
