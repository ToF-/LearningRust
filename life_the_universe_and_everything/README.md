
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
