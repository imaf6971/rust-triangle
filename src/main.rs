use std::io;

fn main() {
    let height: u32 = loop {
        let mut input = String::new();
        println!("How high do you want triangle to be?");
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read from stdin!");
        match input.trim().parse::<u32>() {
            Ok(res) => break res,
            Err(e) => {
                println!("Parsing error! {e}");
                println!("Please, provide an integer greather or equals to zero");
                continue;
            }
        }
    };
    for i in (0..height).rev() {
        for _ in 0..i {
            print!(" ");
        }
        for _ in i..(height - 1) {
            print!("*");
        }
        for _ in (i..height).rev() {
            print!("*")
        }
        println!();
    }
}
