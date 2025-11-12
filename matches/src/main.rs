/*
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    /*
    match coin { // 열거형의 배리언트를 패턴으로 사용하는 match 표현식 -> if 문은 불리언 값을 반환해야하지만, match 는 어떤 타입이든 가능하다.
        Coin::Penny => 1, // 패턴과 실행코드를 구분해주는 => 연산자
        Coin::Nickel => 5, // match 표현식이 실행될 때, 결과값을 각 갈래의 패턴에 대해 순차적으로 비교한다. 만약 어떤 패턴이 그 값과 매칭되면, 그 패턴과 연관된 코드가 실행된다.
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
     */
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1 // 메서드 호출시 Lucky penny! 출력후 블록의 마지막 값인 1을 반환함
        }, // 매치 갈래 내 여러 줄 코드를 실행할 경우, 중괄호를 사용한다. 이 경우 갈래 뒤에 붙이는 쉼표는 옵션이 된다.
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {

}
 */

/*
// 값을 바인딩하는 패턴
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // 변수 state 는 Coin::Quarter 와 매치될때, 값이 바인딩 된다.
            println!("State quarter from {:?}!", state); // 동적으로 변하는 값이므로 {:?} 로 출력한다.
            25
        }
    }
}
 */
/*
// Option<T> 매칭
fn plus_one(x: Option<i32>) -> Option<i32> { // Option<i32> 를 매개변수로 받아서, 내부에 값이 있으면 그 값에 +1 더하는 함수, 만약 내부 값이 없으면 함수는 None 을 반환하고 다른 어떤 연산도 수행하지 않음
    match x {
        None => None, // 매개변수로 None 이 들어올 경우, 해당 프로그램은 멈추고 => 우측 편의 None 값을 반환한다.
        Some(i) => Some(i + 1), // 값이 있을 경우 Some(i) 에 매칭된다. 예를 들어서 Some(5) 인 경우 Some(i) 와 동일한 배리언트 이므로 매칭에 성공한다. Some 내부에 담긴 값은 i 에 바인딩 되므로, i 는 값 5를 갖는다.
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// match 에서 갈래의 패턴들은 모든 가능한 경우(케이스)를 다뤄야 한다. -> 아래 match 는 None 케이스를 다루지 않아 컴파일 오류가 발생한다. -> Rust 의 match 는 철저하다(exhaustive)
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
 */
/*
// 포괄 패턴과 _자리표시자
fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //other => move_player(other), // other 라는 이름을 가진 변수는 나머지 모든 가능한 값을 다루는 마지막 갈래에 대한 패턴이다(값 바인딩). 이를 포괄 패턴이라고 한다. -> 포괄(catch-all)패턴은 마지막에 위치해야 한다.
        // _ => reroll(), // 포괄값 _ 는 어떤 값이라도 매칭되지만, 그 값을 바인딩하지 않는 특별한 패턴이다.
        _ => (), // 앞의 갈래에서 매칭되지 않은 어떠한 값도 사용하지 않을 것이며, 어떠한 코드도 실행하지 않기를 원하다고 명시적으로 알림.
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}
// fn reroll() {}
 */
// match 대신 if let 을 사용한 간결한 제어 흐름
fn main() {
    let config_max = Some(3u8);
    /*
    match config_max {
        Some => println!("The maximum is configured to be {}", max),
        _ => (), // None 값에 대해서 아무런 처리 x -> match 표현식을 만족시키려면 딱 하나의 배리언트만 처리해도 되나 이는 보일러 플레이트 코드이다.
    }
     */
    if let Some(max) = config_max { // if let 은 패턴 = 표현식을 입력받는다. -> match 와 동일한 방식으로 작동 -> 표현식은 match 에 주어진 것이고 패턴은 이 match 의 첫 번째 갈래와 같다. -> 패턴 Some(max) 의 max 는 Some 내부 값에 바인딩 된다 -> if let 본문 블록 내에서 max 사용가능
        println!("The maximum is configured to be {}", max);
    }
}
/*
// if let 은 else 를 포함시킬 수 있다. 아래 match 문과 if let 문은 같은 로직이다.
fn main() {
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
 */
