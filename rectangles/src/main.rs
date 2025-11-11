/*
// 각 변수에 지정된 너비와 높이로 사각형 넓이 계산하기
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "사각형의 넓이: {}", area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// 1. 튜플로 리팩터링: 매개변수 두 값을 하나로 묶어서 관리
fn main() {
    let rect1 = (30, 50); // 튜플로 두 값을 서로 연관시킴

    println!(
        "사각형의 넓이: {}", area(rect1) // 인수를 하나만 넘기면 된다는 점에서 프로그램이 개선됨
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1 // 그러나 각 요소에 이름이 없는 튜플의 특성 때문에, 값을 인덱스로 접근해서 계산식이 불명확해졌다.
}
 */
// 2. 구조체로 리팩터링: 코드에 더 많은 의미를 담기
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { // 구조체 인스턴스 생성
        width: 30,
        height: 50,
    };

    println!(
        "사각형의 넓이: {}", area(&rect1) // 구조체의 소유권을 가져와버리면 더 이상 rect1 을 사용할 수 없으므로, 참조자 타입으로 소유권을 빌려오도록 설정, 불변 참조자 타입이므로 함수 시그니처와 호출 부분에 &을 붙인다
    );
}

fn area(rectangle: &Rectangle) -> u32 { // 구조체 불변 참조자 타입
    rectangle.width * rectangle.height // Rectangle 인스턴스의 필드에 접근(빌린 구조체 인스턴스의 필드에 접근하는 것은 필드값을 이동시키지 않는다), 결과적으로 area 함수의 시그니처가 의미하는 바를 정확히 알려준다.
}

