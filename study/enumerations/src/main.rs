/*
// 1. IP 주소 데이터를 저장할 방법이 없고 어떤 종류인지만 알 수 있음 -> 구조체를 이용하여 배리언트가 연관된 값을 갖도록 함
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
 */
/*
// 2. 열거형 배리언트에 데이터를 직접 넣는 방식 -> 각 열거형 배리언트의 이름이 해당 열거형 인스턴스의 생성자 함수처럼 된다는 것
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}
 */
/*
// 3. 구조체 대신 열거형을 사용할 경우, 각각의 배리언트 다른 타입과 다른 양(amount)와 연관된 데이터를 가질 수 있다.
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
 */
/*
// 4. 배리언트로 열거형을 정의하는 것은 다른 종류의 구조체들을 정의하는 것과 비슷하다.
enum Message {
    Quit, // 연관된 데이터가 전혀 없음
    Move { x: i32, y: i32}, // 구조체처럼 이름이 있는 필드를 가짐
    Write(String), // 하나의 String 을 가짐
    ChangeColor(i32, i32, i32), // 3개의 i32 를 가짐
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 튜플 구조체
struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체
// 구조체에 impl 을 사용해서 메서드를 정의한 것처럼, 열거형에도 정의할 수 있다.
fn main() {
    impl Message {
        fn call(&self) {
            // 메서드 본문 정의
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
 */

/*
Rust 에는 NULL 은 없지만, 값의 존재 또는 값의 부재를 표현할 수 있는 열거형 Option<T> 가 있다.
enum Option<T> {
    None,
    Some(T),
}
Option<T> 는 Rust 에서 기본으로 import 하는 목록인 프렐루드(prelude)에 포함되어 있다. 또, 배리언트도 마찬가지로 프렐루드에 포함되어 있다. 따라서 None, Some 배리언트 앞에 Option:: 을 붙이지 않아도 된다.
 */
fn main() {
    let some_number = Some(5); // Option<i32> 타입, Some 배리언트 내에 어떤 값을 명시했기 때문에 Rust 는 이 타입을 추론할 수 있다.
    let some_char = Some('e'); // Option<char> 타입

    let absent_number: Option<i32> = None; // 반면 absent_number 는 Option 타입을 명시해야 한다. None 값만 봐서는 Some 배리언트가 어떤 타입의 값을 가질지 컴파일러가 추론할 수 없기 때문이다.

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    //let sum = x + y; // Option<i8> 과 i8 은 서로 다른 타입이므로 컴파일 에러 발생, T에 대한 연산을 수행하기 전에 Option<T>를 T로 변환해야 한다. -> 이는 실제로 NULL 인데 NULL 이 아니라고 가정하는 상황을 발견하는데 도움이 된다.
}