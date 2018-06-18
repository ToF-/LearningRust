// http://www.spoj.com/problems/EXPECT/
use std::io:: { stdin, BufRead, BufReader, 
                stdout, Write };

fn main() {
    process(&mut BufReader::new(stdin()))
}

fn process<T:BufRead>(input : &mut T) {
    loop {
        let mut buffer = String::new();

        input.read_line(&mut buffer)
            .expect("input error");

        write!(stdout(), "{}", buffer);

        if buffer == "42\n" {
            break
        }
    }
}

#[cfg(test)]
mod sample_test {

    #[test]
    fn should_show_that_addition_works() {
        assert_eq!(2+2, 4)
    }
}
