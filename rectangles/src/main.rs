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
 */
/*
// Rust 는 디버깅 정보 출력 기능을 가지고 있다. 구조체에 디버깅 출력 기능을 적용할려면 명시적인 동의가 필요하다.
#[derive(Debug)] // 외부 속성
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 는 {:?}", rect1); // {} 내에 :? 을 추가함으로써 Debug 출력 형식 사용, 구조체 내 필드가 많은 경우 {:?} 대신 {:#?}을 사용하면 된다.
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // 표현식을 deg! 로 감싼것은 deg! 가 표현식의 소유권을 반환하면서 dbg! 호출을 하지 않았을 떄와 같은 값이 width 필드에 입력되기 때문.
        height: 50,
    };

    dbg!(&rect1); // Debug 포맷을 사용해서 값을 출력하는 방식, 표현식의 소유권을 가져와서 dbg! 매크로를 호출한 파일과 라인 번호를 결과값과 함께 출력하고 다시 소유권을 반환한다.
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // 구조체 콘텍스트에 함수를 정의하기 위해선, 구조체에 대한 impl 블록을 만들어야 한다.
    fn area(&self) -> u32 { // 메서드의 첫 번째 매개변수는 항상 self 가 된다.
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "사각형의 넓이: {}", rect1.area() // 메서드 문법으로 Rectangle 인스턴스의 area 메서드를 호출할 수 있다.
    );
}
 */
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool { // Rectangle 을 읽기만 하므로 불변 참조자 사용
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}


