fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    expression();

    let x = five();

    println!("x 값: {x}")
}

fn another_function(x: i32, unit_label: char) {
    println!("수치 값: {x}{unit_label}");
}

fn expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("y 값: {y}");
}

fn five() -> i32 {
    5 // 세미콜론이 없으므로 값에 대한 표현식이다.
}