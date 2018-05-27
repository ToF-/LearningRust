// http://www.spoj.com/problems/EXPECT/
use std::io::{
    BufRead,
    BufReader,
    stdin,
    stdout,
    Write
};

fn main() {
    process(&mut BufReader::new(stdin()), &mut stdout());
}

pub fn process<In, Out>(input: &mut In, output: &mut Out)
    where In: BufRead, Out: Write
{
    loop {
        let mut buffer = String::new();
        input.read_line(&mut buffer).expect("input error");
        write!(output, "{}", buffer).expect("output error");
        if buffer == "42\n" {
            break
        }
    }
}

#[cfg(test)]
mod main_process_should {
    use std::io::Cursor;
    use super::*;

    fn assert_process(given: &str, expected: &str) {
        let mut input = Cursor::new(given);
        let mut output= Cursor::new(vec!());

        process(&mut input, &mut output);
        let result = String::from_utf8(output.into_inner()).expect("incorrect utf-8");
        assert_eq!(expected, &result)
    }
    #[test]
    fn stop_after_42_is_read() {
        assert_process("4807\n42\n", "4807\n42\n");
    }
}
