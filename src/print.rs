pub fn number_print(numbers: &[u32; 6]) {
    print!("로또 번호:");
    for i in 0..6 {
        print!(" {}", numbers[i]);
    }
    println!()
}