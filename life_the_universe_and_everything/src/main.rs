// http://www.spoj.com/problems/EXPECT/

use std::io:: {
    stdin,
    Stdin,
    stdout,
    Write
    };

fn main() {
    
    process(stdin(), stdout());

}

fn process(input : Stdin, output : Write) {
    loop {
        let mut buffer = String::new();

        input.read_line(&mut buffer)
            .expect("input error");

        write!(output, "{}", buffer)
            .expect("output error");

        if buffer == "42\n" { 
            break 
        }
    }   
}
