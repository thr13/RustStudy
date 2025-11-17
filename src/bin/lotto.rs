use rust_project::{creation, print, sort, retry};

fn main() {
    println!("로또 번호 추천 프로그램");

    loop {
        let mut numbers = creation::number_creation();
        sort::number_sort(&mut numbers);
        print::number_print(&numbers);

        if !retry::number_retry() {
            println!("프로그램 종료");
            break;
        }
    }
}
