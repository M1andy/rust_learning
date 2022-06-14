fn main() {
    let mut number = 123;
    println!("Tell my the number is {}", number);

    number = 10;
    match number {
        1 => println!("one!"),
        2 | 3 | 4 | 5 | 11 => println!("test number1"),
        13..=19 => println!("test number 2"),
        _ => println!("Ain't special"),
    }
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}