# How to TDD a simple input/output program in Rust

In my project of teaching myself the Rust language, I'm doing the exercises from the list of Basic Problems on the Sphere Online Judge website. 

Here's the one that I picked : 

    EXPECT - Life, the Universe, and Everything (Interactive)

    (from The Sphere Online Judge http://www.spoj.com/problems/EXPECT/)

    Your program is to use the brute-force approach in order to find the Answer to Life, the Universe, and Everything. More precisely... rewrite small numbers from input to output. Stop processing input after reading in the number 42. All numbers at input are integers of one or two digits.

    Interactive Protocol
    You should communicate with Judge using standard input and output.

    Each time the judge will give you a number. You should rewrite this number to standard output. If this number equals 42, after rewriting your program should terminate immediately.

    Example
    The example of communication.

    Input:

        3
        15
        42

    Output:

        3
        15
        42

### A very simple solution

This problem is very easily solved with a small Rust program:

    // http://www.spoj.com/problems/EXPECT/
    use std::io::stdin;

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

It's an infinite loop of reading buffered input and printing it, that breaks only after a certain condition is met. 

By running the program

    cargo run ⏎
    4807 ⏎
    4807
    17 ⏎
    17
    42 ⏎
    42

### The Code and Fix approach
we can see that it works. But this approach to programming:

1. *write a program*
2. *run it to see if it works*

is convenient only work for trivial programs. What if the problem was more complicated ? Then we would very probably be caught in a nasty loop:

1. *write the program*
2. *run the program and look for a failure in its behavior*
4. *find the defect at the origin of the failure*
5. *make changes to the program in order to fix the defect*
6. *goto 2*

Note that *this* infinite loop is not exactly the way the approach is used. We should add a line that breaks the loop on certain conditions :

1. *write the program*
2.  *if really confident or tired or running out of time, then exit*
2. *run the program and look for a failure in its behavior*
4. *find the defect at the origin of the failure*
5. *make changes to the program in order to fix the defect*
6. *goto 2*

As long as we are in this loop, we refrain to make adjustments to the structure of the code, lest we unwittingly insert new defects in the program, leading to new failures in the behavior.

### The Test Driven Development approach
What we want to do instead of this approach, is to follow the Test Driven Development approach:

1. *make a list of all the unit tests that we think our program should pass*
2. *write an automated test for a bit of behavior of the program*
3. *make the test pass with the simplest code that could possibly work*
4. *refactori our code, improving legibility, expressivity and simplicity*
5. *as long as there are tests in our list, goto .2*

That way, we are more confident that the program is running, and that the code has been possibly improved, when we end the loop.

Writing automated unit tests in Rust is easy. Here's one example:

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

What tests can we think of for the program given in the `Life, the universe and everything` exercise ? Let's make a list of them:

1. *(the simplest case) given the line "42" in input, the program will output "42" and then stop.*
2. *(the most frequent case) given some numbers in input, the program will print them until a "42" has been printed, then it will stop.*

With these tests in mind, we can start again writing our program. 

### Testing input and output : a workaround
But then we meet our first difficulty: how do we program a unit test to check what the output of program is, given a specific input ? Good unit tests in TDD are tests that respect the F.I.R.S.T criteria:

- Fast, meaning that manually entering input won't work very well.
- Indepedent, meaning that each test should be executing without having an effect on the execution of others tests.
- Repeatable, which holds only if we take great care of repeating the manual entries without errors. 
- Self-validating, which can't hold if we content ourselves with visually validating the output.
- Timely, meaning we can write each test before writing the part of the program that makes the test pass.

For our tests to be self-validating and repeatable, we can replace manual input with some input data stored in a text file, and then automatically compare the output with some data stored in another text file. Command line tools allow us to do that. Let's create a *test_script* file and run it:
 
    # test_script
    echo "42" >input
    echo "42" >expect
    cargo run <input >output
    diff expect output

    chmod +x test_script ⏎
    ./test_script ⏎
    
The absence of *diff* output means that the test is passing.

Let's have this test execute and fail:

    // http://www.spoj.com/problems/EXPECT/

    fn main() {
    }

    ./test_script  ⏎
    1d0
    < 42

We can make the test pass with a single change:

    // http://www.spoj.com/problems/EXPECT/

    fn main() {
        print!("42\n");
    }

    ./test_script  ⏎

Let's add our second test:

    # test_script
    echo "42" >input
    echo "42" >expect
    cargo run <input >output
    diff expect output

    echo "4807" >input
    echo "42"  >>input
    echo "4807" >expect
    echo "42"  >>expect
    cargo run <input >output
    diff expect output

    ./test_script  ⏎
    1d0
    < 4807

And now we can make it pass with the version we wrote previously:
 
    // http://www.spoj.com/problems/EXPECT/

    use std::io::stdin;

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

    ./test_script  ⏎

### Using seams to test input/output code
This technique could work for a while, but a test written this way  has several limitations :

- It's not really independent, since it relies on the file system.
- It's not really self-validating, since the testing mechanism is not coded in Rust.
- It works only for standard input/output programs.
- It could become very slow given a large amount of input data.

Instead of building such scaffolding around the program, we would prefer to write simple tests in Rust itself, that check bits of functionality, not the whole program each time.

For that to happen, we need to be able to:

- substitute the real input stream with an object in memory that we can initialize to some values.
- substitute the real output stream with an object that we can inspect and compare to our expected values.

In other words, we need *seams* to the input and output streams that our central loop is using. And we need to *command* these seams in our tests (setting the input and output streams to our objects before calling the function) and in the main program (setting the input and output streams to standard input and output).

## Creating a seam on the input stream

Since we want to create seams for the input stream and the output stream, and to be able to command these seams from either the main function of our program, or from a test case, we first need to extract the function that will have the seams as parameters.

Let's start with the input stream. Rust documentation says that `pub fn stdin() -> Stdin` so let's make it an argument:

    use std::io:: {
        stdin,
        Stdin 
        };

    fn main() {
        
        process(stdin());

    }

    fn process(input : Stdin) {
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
    
    ./test_script  ⏎

### Creating a seam on output

Creating the seam for the output stream is a bit more indirect, as the `print!` macro makes this output stream invisible. `print!` can be replaced with the `write!` macro. 


    // http://www.spoj.com/problems/EXPECT/

    use std::io:: {
        stdin,
        Stdin,
        stdout
        };

    fn main() {
        
        process(stdin());

    }

    fn process(input : Stdin) {
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

The rust compiler will notify us as to what should be imported:

    error[E0599]: no method named `write_fmt` found for type `std::io::Stdout` in the current scope
      --> src/main.rs:22:9
       |
    22 |         write!(stdout(), "{}", buffer);
       |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
       |
       = help: items from traits can only be used if the trait is in scope
       = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
    help: the following trait is implemented but not in scope, perhaps add a `use` for it:
       |
    3  | use std::io::Write;
       |

We import the `Write` trait in the scope where `write!` is used.

    use std::io:: {
        stdin,
        Stdin,
        stdout,
        Write
        };

    fn main() {
        
        process(stdin());

    }

    fn process(input : Stdin) {
        loop {
            let mut buffer = String::new();

            input.read_line(&mut buffer)
                .expect("input error");

            write!(stdout(), "{}", buffer)
                .expect("output error");

            if buffer == "42\n" { 
                break 
            }
        }   
    }

Now we make `stdout()` a parameter value for the `process` function. The documentation says that this macro is typically used with a buffer of `&mut Write` so let's use that:

    // http://www.spoj.com/problems/EXPECT/
    use std::io:: {
        stdin,
        Stdin,
        stdout,
        Write
    };

    fn main() {

        process(stdin(), &mut stdout());

    }

    fn process(input : Stdin, output : &mut Stdout) {
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

There! We created two seams for input and output streams.

### Substituting a buffer to the input stream.

Let's try, for the sake of experimenting, to substitute the input stream with a buffer with hard-coded data.

We know that `stdin() -> Stdin` implements the `Read` trait. Can we substitute it with an object that would also implement this trait ? Basically what we need is an object in memory that would implement the `read_line` method. Classes that implement the trait `BufRead` have a `read_line` method. Thus we need to change the `process` function so that it can take a value of any type that implements the trait `BufRead` and then we can use the BufReader class, that provides a buffered read, given a buffer source:

    use std::io:: {
        stdin,
        stdout,
        Write,
        BufRead,
        BufReader

    };

    fn main() {

        process(&mut BufReader::new(stdin()), &mut stdout());

    }

    fn process<T:BufRead>(input : &mut T, output : &mut Write)
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

Now, what if we gave another buffer instead of `stdin` to our `BufReader`? Let's give it a `Cursor`, that we'll initialize with a hard coded string.

    fn main() {

        process(
            &mut BufReader::new(Cursor::new("6\n6\n6\n42\n")),
            &mut stdout());

    }

And now the tests reveal that the input has been forced to 6s :

    0a1,3
    > 6
    > 6
    > 6

    1c1,3
    < 4807
    ---
    > 6
    > 6
    > 6

### Substituting a buffer to stdout()

If we put a `Cursor` initialized with a `vec` in the place ofi `stdout()`, 


    fn main() {

        process(
            &mut BufReader::new(Cursor::new("6\n6\n6\n42\n")),
            &mut Cursor::new(vec!()));

    }

then the tests reveal that the program didn't print anything :

    1d0
    < 42

    1,2d0
    < 4807
    < 42
    < 42

### Writing automated test with the seams

Now that we know we can manipulate seams, we can use them for testing. The idea is to :

1. set up an input buffer with a hard coded string, e.g. `"4807\n42\n"`
2. set up the output buffer to a new `Vec` 
3. call our `process` function with these parameters
4. get the content of the output buffer
5. compare this content with an expected result (here `"4807\n42\n"`) 

Step 4  is done using  `into_inner()` to access the inner representation of the buffer, and `from_utf8` to convert the utf-8 codes into a standard string.


    #[cfg(test)]
    #[test]
    fn main() {

        let input = Cursor::new("4807\n42\n");
        let mut output= Cursor::new(vec!());
        process(&mut BufReader::new(input), &mut output);
        
        let result = String::from_utf8(output.into_inner())
            .expect("incorrect utf-8");

        assert_eq!("4807\n42\n", result);

    }

And now we can run our program as a test suite: 

    cargo test ⏎

       Doc-tests life_the_universe_and_everything

    running 0 tests

    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

What if we sabotage our `process`function ?

    fn process<T:BufRead>(input : &mut T, output : &mut Write)
    {
        loop {
            let mut buffer = String::new();

            input.read_line(&mut buffer)
                .expect("input error");

            write!(output, "{}!", buffer)
                .expect("output error");

            if buffer == "42\n" { 
                break 
            }
        }   
    }


Then the test reveal the problem:

    cargo test ⏎
    thread 'main' panicked at 'assertion failed: `(left == right)`
      left: `"4807\n42\n"`,
     right: `"4807\n!42\n!"`', src/main.rs:23:5
    failures:
        main

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

### Commanding the seams

Now that we have everything we need to unit test a function that uses input and output streams, we can put our tests on one side, and keep our main program using this function normally :

    // http://www.spoj.com/problems/EXPECT/
    use std::io:: {
        stdin,
        stdout,
        Write,
        BufRead,
        BufReader,

    };

    fn main() {

        process(&mut BufReader::new(stdin()), &mut stdout());
        
    }

    fn process<T:BufRead>(input : &mut T, output : &mut Write)
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
        
        #[test]
        fn output_its_input_until_42_is_printed() {
            let input = Cursor::new("4807\n42\n");
            let mut output= Cursor::new(vec!());
            process(&mut BufReader::new(input), &mut output);
            
            let result = String::from_utf8(output.into_inner())
                .expect("incorrect utf-8");

            assert_eq!("4807\n42\n", result);
        }

    }

And now we can refactor our tests, using helpers.

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
