fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition is true");
    } else {
        println!("condigtion is false")
    }

    match number {
        3 => println!("three"),
        _ => println!("not 3"),
    }

    fn for_loop(x: [i32; 3]) -> i32 {
        for y in x.iter() {
            println!("{}", y);
        }
        for y in (1..4).rev() {
            println!("{}", y);
        }
        2
    }
    for_loop([1, 2, 3]);
}
