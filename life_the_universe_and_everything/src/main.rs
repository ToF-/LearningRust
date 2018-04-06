fn main() {
    use std::io;

    fn read_input() -> io::Result<String> {
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;


        Ok(input)
    }
    let mut done = false;
    while !done {
        match read_input() {
            Ok(line) => { 
                let input = line.trim();
                println!("{}", input);
                done = input == "42"
            }
            Err(e) => {
                println!("{}",e);
            }
        }
    }

}
