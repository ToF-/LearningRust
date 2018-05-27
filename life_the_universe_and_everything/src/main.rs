// http://www.spoj.com/problems/EXPECT/
use std::io::{
    stdin,
};

fn main() {
//    process(&mut BufReader::new(stdin()), &mut stdout());
    loop {
        let mut buffer = String::new();

        stdin().read_line(&mut buffer)
            .expect("input error");

        print!("{}", buffer);

        if buffer == "42\n" {
            break
        }
    }
}

// pub fn process<In, Out>(input: &mut In, output: &mut Out)
//     where In: BufRead, Out: Write
// {
//     loop {
//         let mut buffer = String::new();
// 
//         input.read_line(&mut buffer).expect("input error");
// 
//         write!(output, "{}", buffer).expect("output error");
//         if buffer == "42\n" {
//             break
//         }
//     }
// }
// 

#[cfg(test)]
mod sample_test {

    #[test]
    fn should_show_that_addition_works() {
        assert_eq!(2+2, 4)
    }

}

// mod main_process_should {
//     use std::io::Cursor;
//     use super::*;
// 
//     fn given_then_expect(given: &str, expected: &str) {
//         let mut input = Cursor::new(given);
//         let mut output= Cursor::new(vec!());
// 
//         process(&mut input, &mut output);
// 
//         let result = String::from_utf8(output.into_inner()).expect("incorrect utf-8");
// 
//         assert_eq!(expected, &result)
//     }
//     #[test]
//     fn stop_after_42_is_read() {
//         given_then_expect("4807\n42\n", 
//                           "4807\n42\n");
//     }
// }
