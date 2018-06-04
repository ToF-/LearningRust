# EXPECT - Life, the Universe, and Everything (Interactive)

(from The Sphere Online Judge http://www.spoj.com/problems/EXPECT/)
Your program is to use the brute-force approach in order to find the Answer to Life, the Universe, and Everything. More precisely... rewrite small numbers from input to output. Stop processing input after reading in the number 42. All numbers at input are integers of one or two digits.

## Interactive Protocol
You should communicate with Judge using standard input and output.

Each time the judge will give you a number. You should rewrite this number to standard output. If this number equals 42, after rewriting your program should terminate immediately.

## Example
The example of communication.

### Input:

    3
    15
    42

### Output:

    3
    15
    42

It's an easy problem to solve with a program in Rust.

    // http://www.spoj.com/problems/EXPECT/
    use std::io::{
        stdin,
    };

    fn main() {
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

We run it, and see what happens:

    cargo run ⏎
    4807 ⏎
    4807
    17 ⏎
    17
    42 ⏎
    42

It's working. But this approach to programming:

    - writing a program
    - running it to see if it works

can only work for very simple programs like this one.

What if the problem was more complex ? Then we would very probably be caught in a nasty loop:

1. writing the program
2. running it to see if it works
3. noticing a failure in the behavior of the program
4. finding the defect at the origin of the failure
5. making changes to the program in order to fix the defect
6. goto 1

This loop stops only when we are sure, after step 2, that our program is working correctly.
As long as we are in this loop, we refrain to make adjustments to the structure of the code, lest we unwittingly insert new defects in the code, leading to new failures in the behavior.

Instead, we want to write our programs with the following approach:

1. making a list of all the unit tests that we think our program should pass
2. writing an automated test for a bit of behavior of the program
3. making the test pass with the simplest code that wpossibly work
4. refactoring our code, improving legibility, expressivity and simplicity
5. as long as there are tests in our list, goto .2

Writing an automated unit test is easy. Here's one example:

    #[cfg(test)]
    mod sample_test {

        #[test]
        fn should_show_that_addition_works() {
            assert_eq!(2+2, 4)
        }

    }

Here's how we run it:

    cargo test ⏎
      running 1 test
    test tests::it_works ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


Let's try this approach with our simple problem.

Making a list of the test our program should pass:

1. the simplest case: given the line "42" in input, the program will output "42" and then stop.
2. the most current case: given some line in input, the program will print them until a "42" is printed, then it will stop.

How to we program a unit test to check what the output of a program is, given a specific input ?
Tests using standard input and output are not reliable, unless we mean a test that is automated from the command line:

    echo "42" >>input
    echo "42" >>expect
    cargo run <input >output
    diff expect output     

This method could work for our simple program, but it involves building "scaffolding" script outside the code of our program, and it only works for testing the whole behavior, not bit of behavior.

Ideally we would prefer to be able to tell our program that the input to the program is not coming from the standard input stream, but from an object in memory that we have defined ourselves. In the same way, we would like to "reroute" the output of the program into an object in memory that we can examise within a test.

In other words, we need *seams* to the input and output channels used by out program.

Substituting to stdin()

    impl Stdin
    
    pub fn read_line(&self, buf: &mut String) -> Result<usize>

    Locks this handle and reads a line of input into the specified buffer.

    For detailed semantics of this method, see the documentation on BufRead::read_line.

A BufRead is a type of Reader which has an internal buffer, allowing it to perform extra ways of reading.

A Cursor wraps another type and provides it with a Seek implementation.

Cursors are typically used with in-memory buffers to allow them to implement Read and/or Write, allowing these buffers to be used anywhere you might use a reader or writer that does actual I/O.

The standard library implements some I/O traits on various types which are commonly used as a buffer, like Cursor<Vec<u8>> and Cursor<&[u8]>.


    use std::io::{
        stdin,
        Cursor,
        BufRead,
    };

    fn main() {
        let input = &mut Cursor::new("4807\n42\n");
        loop {
            let mut buffer = String::new();

            input.read_line(&mut buffer)
                .expect("input error");

            print!("{}", buffer);

            if buffer == "42\n" {
                break
            }
        }
    }


    cargo run ⏎
    4807
    42

Substituting to stdout()


    use std::io::{
        stdin,
        stdout,
        Cursor,
        BufRead,
        Write,
    };

    fn main() {
        let mut input = Cursor::new("4807\n42\n");
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



    use std::io::{
        stdin,
        stdout,
        Cursor,
        BufRead,
        Write,
    };

    fn main() {
        let input = &mut Cursor::new("4807\n42\n");
        let mut output= Cursor::new(vec!());

        loop {
            let mut buffer = String::new();

            input.read_line(&mut buffer)
                .expect("input error");

            write!(output, "{}", buffer);

            if buffer == "42\n" {
                break
            }
        }
        print!("{}",String::from_utf8(output.into_inner()).expect("incorrect utf-8"));
    }
`
And now we extract a method:

    fn main() {
        let mut input = Cursor::new("4807\n42\n");
        let mut output= Cursor::new(vec!());
        process(&mut input, &mut output);
        print!("{}",String::from_utf8(output.into_inner()).expect("incorrect utf-8"));
    }

    pub fn process<In,Out>(input: &mut In, output: &mut Out)
        where In: BufRead, Out: Write
    {
        loop {
            let mut buffer = String::new();

            input.read_line(&mut buffer)
                .expect("input error");

            write!(output, "{}", buffer);

            if buffer == "42\n" {
                break
            }
        }
    }

Restablish the main program :

    use std::io::{
        stdin,
        stdout,
        Cursor,
        BufRead,
        BufReader,
        Write,
    };

    fn main() {
        process(&mut BufReader::new(stdin()), &mut stdout());
    }

    pub fn process<In,Out>(input: &mut In, output: &mut Out)
        where In: BufRead, Out: Write
    {
        loop {
            let mut buffer = String::new();

            input.read_line(&mut buffer)
                .expect("input error");

            write!(output, "{}", buffer);

            if buffer == "42\n" {
                break
            }
        }
    }

Our first test: 

    #[cfg(test)]
     mod main_process_should {
         use std::io::Cursor;
         use super::*;

        #[test]
        fn given_42_print_42_then_stop() {

            let mut input = Cursor::new("42\n");
            let mut output = Cursor::new(vec!());

            process(&mut input, &mut output);
            
            let result = String::from_utf8(output.into_inner())
                .expect("incorrect utf-8");

            assert_eq!(result, "42\n");
        }
    }

A second test:

        #[test]
        fn given_any_input_stop_after_42() {

            let mut input = Cursor::new("4807\n42\n");
            let mut output = Cursor::new(vec!());

            process(&mut input, &mut output);
            
            let result = String::from_utf8(output.into_inner())
                .expect("incorrect utf-8");

            assert_eq!(result, "4807\n42\n");
        }

Refactoring the tests

    #[cfg(test)]
     mod main_process_should {
         use std::io::Cursor;
         use super::*;

         fn given_then_expect(given: &str, expected: &str) {
             let mut input = Cursor::new(given);
             let mut output= Cursor::new(vec!());
     
             process(&mut input, &mut output);
     
             let result = String::from_utf8(output.into_inner())
                .expect("incorrect utf-8");
     
             assert_eq!(expected, &result)
         }
        #[test]
        fn given_42_print_42_then_stop() {
            given_then_expect("42\n","42\n");
        }
        #[test]
        fn given_any_input_stop_after_42() {
            given_then_expect("4807\n42\n","4807\n42\n");
        }
    }

Et voilà.
