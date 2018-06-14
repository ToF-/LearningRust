// http://www.spoj.com/problems/EXPECT/
use std::io:: {
    stdin,
    Stdin,Stdout,
    stdout,
    Write,
    BufRead,
    BufReader,

};

fn main() {

    process(&mut BufReader::new(stdin()), &mut stdout());
    processp(stdin(), &mut stdout());
    
}

fn processp(input : Stdin, output : &mut Stdout) {
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
fn process(input : &mut BufRead, output : &mut Write)
{
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

#[cfg(test)]
mod process_should {
    use super::*;
    use std::io::Cursor;

    fn given_expect(given : &str, expected : &str) {
        let input = Cursor::new(given);
        let mut output= Cursor::new(vec!());
        process(&mut BufReader::new(input), &mut output);
        
        let result = String::from_utf8(output.into_inner())
            .expect("incorrect utf-8");

        assert_eq!(expected, result);
    }
    
    #[test]
    fn output_42_if_only_given_42() {
        given_expect("42\n", "42\n");
    }
    #[test]
    fn output_its_input_until_42_is_printed() {
        given_expect("4807\n42\n", "4807\n42\n");
    }

}
