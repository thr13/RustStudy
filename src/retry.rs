use std::io;

pub fn number_retry() -> bool {
    loop {
        println!("다시 추천 받기를 원하신다면 Y를, 프로그램을 종료하길 원하신다면 N 를 입력해주세요.");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("잘못된 입력입니다.");

        match input.trim() {
            "y" | "Y" => return true,
            "n" | "N" => return false,
            _ => {
                println!("잘못된 입력입니다. Y 또는 N 만 입력해주세요.\n");
                continue;
            }
        }
    }
}