pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    numbers[2]=44;
    println!("{:?}", numbers);
}
