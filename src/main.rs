mod color_space;

use std::io;

use color_space::hsl_fixed_l;
use color_space::rgb;

fn main() {
    let mut input = String::new();

    println!("This program outputs some key colors in the hsl and rgb color space.");
    println!("The size of the rgb colorspace can be changed.");
    println!("How many colors do you want to see?");
    println!(
        "
        Press 1:   2  colors per rgb channel
        Press 2:   4  colors per rgb channel
        Press 3:   8  colors per rgb channel
        Press 4:  16  colors per rgb channel
        Press 5:  32  colors per rgb channel
        Press 6:  64  colors per rgb channel
        Press 7: 128  colors per rgb channel
        Press 8: 256  colors per rgb channel
    "
    );
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: u32 = match input.trim().parse() {
        Ok(num) => num,

        Err(_) => {
            println!("The program only accept numbers!");
            return;
        }
    };

    match number {
        1..=8 => {
            hsl_fixed_l(5, 5);
            rgb(number as i32);
        }

        _ => {
            println!("That is a invalid number!");
        }
    }
}
