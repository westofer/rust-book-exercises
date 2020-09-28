// TODO:
use std::io;

fn main() {
    let temp: i32 = loop {
        let mut temp = String::new();

        println!("Input a number");
        io::stdin()
            .read_line(&mut temp)
            .expect("Couldn't read Input");

        let temp: i32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You want to convert: {}", temp);
        break temp;
    };

    let unit: char = loop {
        println!("Unit (F/C)");
        let mut unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("Couldn't read Input");

        let input = unit.trim().chars().next().unwrap();
        let input = match input {
            'F' | 'f' => 'F',
            'C' | 'c' => 'C',
            _ => continue,
        };
        break input;
    };

    let mut converted: f32;
    if unit == 'F' {
        converted = feh_to_celsius(&temp);
    } else {
        converted = celsius_to_feh(&temp);
    }
    println!("Unit :{}", unit);
    println!("-32* (5/9) {}", (-32) * (5 / 9));
    println!("converted {}", converted);
}

fn feh_to_celsius(x: &i32) -> f32 {
    *x as f32 * (9.0 / 5.0) + 32.0
}
fn celsius_to_feh(x: &i32) -> f32 {
    (*x as f32 - 32.0) * (5.0 / 9.0)
}
