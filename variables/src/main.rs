fn main() {
    fn nameless(x: i32) -> i32 {
        x
    }

    let x = 5;
    println!("The value of x is: {}", x);

    let x = 6;
    println!("The value of x is: {}", x);

    let y = true;
    println!("The value of x is: {}", y);
    println!("x: {}", nameless(8))
}
