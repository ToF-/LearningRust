
fn main() {
    use std::io;

    fn print_pattern(rows : u32, cols : u32) {
        for row in 0..rows {
            for col in 0..cols {
                if  ((row%2) + (col%2))%2 == 0 {
                    print!("*")
                }
                else {
                    print!(".")
                }
            }
            println!()
        }
    }
    fn read_input() -> io::Result<String> {
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;


        Ok(input)
    }

    let mut done = false;
    let max = read_input().expect("no input").trim().parse().expect("not a number");
    for l in 0..max {
        match read_input() {
            Ok(line) => { 
                let inputs: Vec<u32> = line.trim().split(" ")
                    .map(|x| x.parse().expect("Not an integer!"))
                    .collect();
                print_pattern(inputs[0], inputs[1]);
                println!("")
            }
            Err(e) => {
                done = true;
            }
        }
    }
}
