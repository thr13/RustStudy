fn main() {
    // loops();

    // loop_labels();

    // whiles();

    // fors();

    fors_pattern();
}

fn loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("결과: {result}");
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("종료시 count 값: {count}");
}

fn whiles () {
    let mut number = 3;

    while number != 0 {
        println!("number 값: {number}");
        number -= 1;
    }
    println!("LIFE OFF!!")
}

fn fors () {
    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("배열 값: {element}");
    }

    for number in (1..4).rev() { // 1 부터 n-1 까지 반복한다. 여기서는 범위를 역순으로 하는 rev() 메소드가 적용되어 n-1 부터 1 까지 반복함
        println!("number 값: {number}")
    }
    println!("LIFE OFF!!")
}

fn fors_pattern () {
    let pairs = [(1, 2), (3, 4), (5, 6)];

    for (a, b) in pairs {
        println!("a = {a}, b = {b}");
    }

}