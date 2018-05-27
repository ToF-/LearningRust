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
6. goto .w

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

That method could work for our simple program, but involves scaffolding to be built outside the code of our program, and it only works for testing the whole behavior, not bit of behavior.

We need to be able to tell our program that the "input" is not coming from the standard input stream, but from an object in memory that we have defined ourselves.

