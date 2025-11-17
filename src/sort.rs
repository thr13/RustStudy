pub fn number_sort(numbers: &mut [u32; 6]) {
    for i in 0..6 {
        for j in 0..6-i-1 {
            if numbers[j] > numbers[j+1] {
                let temp = numbers[j];
                numbers[j] = numbers[j+1];
                numbers[j+1] = temp;
            }
        }
    }
}