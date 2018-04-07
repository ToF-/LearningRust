fn main() {
    use std::io;

    fn print_pattern(rows : u32, cols : u32, height : u32, width : u32) {
        for row in 0..rows*(height+1)+1 {
            for col in 0..cols*(width+1)+1 {
                if (row % (height+1) == 0) || (col % (width+1) == 0)  { 
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

    let max = read_input().expect("no input").trim().parse().expect("not a number");
    for _ in 0..max {
        match read_input() {
            Ok(line) => { 
                let inputs: Vec<u32> = line.trim().split(" ")
                    .map(|x| x.parse().expect("Not an integer!"))
                    .collect();
                print_pattern(inputs[0], inputs[1],inputs[2],inputs[3]);
                println!("")
            }
            Err(_) => {
                break
            }
        }
    }
}
