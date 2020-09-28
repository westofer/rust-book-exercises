use std::io;

fn main() {
    let input: i32 = loop {
        let mut input = String::new();
        println!("Fib finder, Enter a number");
        io::stdin().read_line(&mut input).expect("bruh");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break input;
    };
    let fibSeq: i32 = fib(input);
    println!("Fib seq: {}", fibSeq);
}

fn fib(x: i32) -> i32 {
    if x == 1 || x == 0 {
        return x;
    }
    return fib(x - 1) + x;
}
