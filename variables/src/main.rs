fn main() {
    /*
    // 변수
    let mut x = 5;
    println!("x 값: {x}");
    x = 6;
    println!("x 값: {x}");
     */

    // 새도잉
    let x = 5; // 변수 x 에 값 5를 바인딩
    let x = x + 1; // x 의 최종값은 6
    { // 중괄호 {} 로 만든 안쪽 스코프
        let x = x * 2; // x 의 최종값은 12
        println!("변수 x 의 값 내부 스코프: {x}");
    } // 안쪽 스코프 종료시 안쪽 섀도잉은 종료되므로 x 의 값은 6 으로 돌아옴
    println!("x 값: {x}");
}
