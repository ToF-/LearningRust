fn main() {
    use std::io;

    fn print_half_of_half(s:& str) {
        let mut counter = 0;
        let max = s.len() / 2;
        for c in s.chars() {
            if counter % 2 == 0 {
                print!("{}",c)
            };
            counter+=1;
            if counter >= max { break }
        };
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
                print_half_of_half(line.trim());
                println!("")
            }
            Err(e) => {
                done = true;
            }
        }
    }
}
