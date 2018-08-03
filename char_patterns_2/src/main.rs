// http://www.spoj.com/problems/CPTTRN2/
use std::io::{
    stdin,
    stdout,
    Result,
    Write,
};
pub fn print_pattern<Out> (rows : u32, cols : u32, output: &mut Out) where Out: Write
{
    for row in 0..rows {
        for col in 0..cols {
            write!(output,"{}",
                   pattern(on_frame(row,col,rows,cols)))
                   .expect("output error")
                   }
        write!(output, "\n").expect("output error");
    }
}

pub fn on_frame(row: u32, col: u32, max_row: u32, max_col: u32) -> bool {
    row == 0 || col == 0 || row == max_row-1 || col == max_col-1
}

pub fn pattern(value : bool) -> String {
    if value {
        String::from("*") 
    } else {
        String::from(".")
    }
}

fn main() {
    use std::io;

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
                print_pattern(inputs[0], inputs[1], &mut stdout());
                println!("")
            }
            Err(_) => {
                break
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
    fn given_3_and_5_print_a_frame() {
        given_then_expect(3,5,"*****\n*...*\n*****\n");
    }
 }
#[cfg(test)]
 mod frame_limit {
    use super::*;

    #[test]
    fn given_coords_and_max_tells_if_on_frame() {
        assert_eq!(on_frame(0,0,3,5),true);
        assert_eq!(on_frame(1,1,3,5),false);
        assert_eq!(on_frame(1,0,3,5),true);
        assert_eq!(on_frame(2,1,3,5),true);
        assert_eq!(on_frame(1,4,3,5),true);
    }
    #[test]
    fn given_a_bool_gives_a_dot_or_a_star() {
        assert_eq!(pattern(false),String::from("."));
        assert_eq!(pattern(true),String::from("*"));
    }
 }

