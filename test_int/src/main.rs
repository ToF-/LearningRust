use std::io;
fn main() {
    let x : i32;
    let y : i32;
    let mut x_string = String::new();
    let mut y_string = String::new();
    io::stdin().read_line(&mut x_string).expect("failed to read from stdin");
    io::stdin().read_line(&mut y_string).expect("failed to read from stdin");
    x = x_string.trim().parse::<i32>().expect("this is not an integer");
    y = y_string.trim().parse::<i32>().expect("this is not an integer");
    println!("{}",x+y)
}

