// http://www.spoj.com/problems/EXPECT/
use std::io:: {
    stdin,
    Stdin,
    stdout,
    Write,
    Cursor,
    BufRead
};

fn main() {

    let cursor = &mut Cursor::new("4807\n42\n");
    process(stdin(), &mut stdout());

}

fn process(input : &mut BufRead, output : &mut Write) {
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

