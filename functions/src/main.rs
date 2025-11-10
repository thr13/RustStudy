fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    expression();
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