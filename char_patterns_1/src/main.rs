// http://www.spoj.com/problems/CPTTRN1/
use std::io::{
    stdin,
    stdout,
    Result,
    Write,
};
pub fn print_pattern<Out>(rows : u32, cols : u32, output: &mut Out)
    where Out: Write
{

    for row in 0..rows {
        for col in 0..cols {
            if  (row + col)%2 == 0 {
                write!(output, "*").expect("output error");
            }
            else {
                write!(output, ".").expect("output error");
            }
        }
        write!(output, "\n").expect("output error");
    }
}

fn main() {

    fn read_input() -> Result<String> {
        let mut input = String::new();

        stdin().read_line(&mut input)?;


        Ok(input)
    }

    let max = read_input().expect("no input").trim().parse().expect("not a number");
    for _ in 0..max {
        match read_input() {
            Ok(line) => { 
                let inputs: Vec<u32> = line.trim().split(" ")
                    .map(|x| x.parse().expect("Not an integer!"))
                    .collect();
                print_pattern(inputs[0], inputs[1], &mut stdout());
                println!("")
            }
            Err(_) => {
                break;
            }
        }
    }
}
#[cfg(test)]
 mod print_pattern_should {
    use std::io::Cursor;
    use super::*;

    fn given_then_expect(rows: u32, cols:u32, expected: &str) {
        let mut output = Cursor::new(vec!());
        print_pattern(rows, cols, &mut output);
        let result = String::from_utf8(output.into_inner())
            .expect("incorrect utf-8");
        assert_eq!(expected, result);
    }

    #[test]
    fn given_1_and_1_print_a_star() {
        given_then_expect(1,1, "*\n");
    }
    #[test]
    fn given_3_and_1_print_a_star_a_dot_and_a_star() {
        given_then_expect(3,1, "*\n.\n*\n");
    }
    #[test]
    fn given_2_and_5_print_a_specifc_pattern(){
        given_then_expect(2,5, "*.*.*\n.*.*.\n");
    }
 }
