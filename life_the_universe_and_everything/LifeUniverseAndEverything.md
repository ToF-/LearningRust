# Life, the Universe and Everything
How to unit test a simple input/output program in Rust.

In my project of teaching myself the Rust language, I'm doing the exercises from the list of  [Basic Problems](https://www.spoj.com/problems/basics/) on the Sphere Online Judge website. 

Here's [the one that I picked](https://www.spoj.com/problems/EXPECT/)  : 

    EXPECT - Life, the Universe, and Everything (Interactive)

    Your program is to use the brute-force approach in order to find the Answer to Life, the Universe, and Everything. More precisely... rewrite small numbers from input to output. Stop processing input after reading in the number 42. All numbers at input are integers of one or two digits.

    You should communicate with Judge using standard input and output.

    Each time the judge will give you a number. You should rewrite this number to standard output. If this number equals 42, after rewriting your program should terminate immediately.

    Example
    Input:
        3
        15
        42

    Output:
        3
        15
        42

### 1. AC in one go!

Here's a rust program that when submitted is greeted with an AC (meaning accepted) status:

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

Here's what we get when running the program:

    cargo run ⏎
    4807 ⏎
    4807
    17 ⏎
    17
    42 ⏎
    42

### 2. Code and Fix vs Test Driven approach
we can see that it works. But this approach to programming:

1. *write a program*
2. *run it to see if it works*

is convenient only for trivial programs. What if the problem was more complicated ? Then we would very probably be caught in *this* nasty loop:


1. *write the program*
2.  *if really confident or tired or running out of time, then exit*
2. *run the program and look for a failure in its behavior*
4. *find the defect at the origin of the failure*
5. *make changes to the program in order to fix the defect*
6. *goto 2*

Notice the exit condition.  As long as we are in this loop, we refrain to make adjustments to the structure of the code, lest we unwittingly insert new defects in the program, leading to new failures in the behavior.

What we want to do instead of this loop, is to follow a sounder approach:

1. *make a list of all the unit tests that we think our program should pass*
2. *write an automated test for a bit of behavior of the program*
3. *make the test pass with the simplest code that could possibly work*
4. *refactor our code, improving legibility, expressivity and simplicity*
5. *as long as there are tests in our list, goto .2*

That way, we are more confident that the program is running, and that the code has been possibly improved when we end the loop.

Writing automated unit tests in Rust is easy. We just add a test section to our program:

    #[cfg(test)]
    mod sample_test {

        #[test]
        fn should_show_that_addition_works() {
            assert_eq!(2+2, 4)
        }
    }

And here's how we run the test:

    cargo test ⏎
    running 1 test
    test sample_test::should_show_that_addition_works ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

What tests can we think of for the program given as an exercise ?

1. *(the simplest case) given the line "42" in input, the program will output "42" and then stop.*
2. *(the most frequent case) given some numbers in input, the program will print each of them, until a "42" is given, then it will print 42, then it will stop.*

We could argue that the case where the input is not numeric should be added, but after all, that's not part of the spec. So, with these tests in mind, we can write our program again, using the TDD aproach.

### 4. Testing input and output : a workaround

Or can we? The macro `assert_eq` allows us to compare two expressions, but it can't possibly help us in asserting that a *sequence of events happened*, and happened on the standard input and output flow, for that matter.

The TDD approach states that the unit tests we create should be fast, independant, repeatable, self-validating and timely (i.e. each written before the code that makes it pass). 
  
For our tests to be self-validating and repeatable, we can replace manual input with some numbers stored in a text file, and then automatically compare the output of our program with some expected results stored in another text file. It's a very convenient way to test, since our rust program is running on an OS where having text files replace standard input and output is easy, as is comparing text files.

Here's a little script doing that for us:

    # test_script

    # when given 42 should print 42 and then stop
    echo "42" >input
    echo "42" >expect
    cargo run <input >output
    diff expect output

    chmod +x test_script ⏎
    ./test_script ⏎
    
`diff` will display the differences between the output and the exepected result if any. The absence of any *diff* output means that the test is passing.

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

Silent diff! Our test passes.

Let's add our second test:

    # test_script

    # when given 42 should print 42 and then stop
    echo "42" >input
    echo "42" >expect
    cargo run <input >output
    diff expect output

    # when given numbers, should print them until 42 is printed
    echo "4807" >input
    echo "42"  >>input
    echo "4807" >expect
    echo "42"  >>expect
    cargo run <input >output
    diff expect output

    ./test_script  ⏎
    1d0
    < 4807

The diff output signals that 4807 was expected, but not present in the result.

Let's make this second test pass, with our initial version of the program:
 
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

Ok! Our program is complete, and well tested, at least on the cases that matter to us.

### 5. But, you're not using unit test in Rust ?

This technique could work for a while, but tests written this way  has several limitations :

- They maybe fast in execution, but writing them is tedious.
- They are not really independant, since they rely on the file system.
- They are not really self-validating, since they are not coded in Rust.
- They work only for a program that uses standard input and outputs.

Instead of building such scaffolding around the program, we would rather write simple tests in Rust itself, tests that do not rely on any file, that check bits of functionality, and eventually do not necessarily involve running the whole program each time.

So we are back at our initial problem: how do we test that something has happened on the output stream, because of something we forced on the input stream?

The answer is: we don't. Instead we do exactly as we did with the scaffolding script : 
- substitute the real input stream with an object in memory that we can initialize to some values
- substitute the real output stream with an object that we can later inspect and compare to our expected values

In other words, we create *seams* on the input and output streams that are used by our central loop. Then we create *commands* on those seams, giving us the ability to switch from memory object to the real stream depending on the configuration : 
- our main program should set the seams to the standard input and output
- our tests should set the seams to object in memory.

The questions is: what kind of objects do we need ?

## 6. Creating a seam on the input stream

Since we want to create seams for the input stream and the output stream, and to be able to command these seams from either the main function of our program, or from a test case, we first need to extract the function that will have the seams as parameters.

    use std::io::stdin;

    fn main() {
        process()
    }

    fn process() {
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

    ./test_script ⏎
    
The silent diff tells us that this refactoring very probably didn't alter the behavior of the program.
    
Let's start with input stream, make stdin() an actual parameter for the `process` function.

    use std::io:: { stdin, Stdin };

    fn main() {
        process(stdin())
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

### 7. Substituting a buffer to the input stream.

Let's try, for the sake of experimenting, to substitute the input stream with an object in memory, namingly, a buffer containing some hard-coded string.

We know that `Stdin` implements the `Read` trait. Can we substitute our parameter  with an object that would also implement this trait ? Basically what we need is an object in memory that would implement the `read_line` method. Classes that implement the trait `BufRead` have a `read_line` method. One of them is `BufReader`. Here's an example of using one:

    use std::io::BufReader;
    use std::fs::File;

    fn main() -> std::io::Result<()> {
        let f = File::open("log.txt")?;
        let reader = BufReader::new(f);
        Ok(())
    }

This is exactly what we need! So let's try it:

    use std::io:: { stdin, Stdin, BufRead, BufReader };

    fn main() {
        process(&mut BufReader::new(stdin()))
    }

    fn process<T:BufRead>(input : &mut T) {
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

We have changed the `process` function so that it can take a value of any type that implements the trait `BufRead`. We have a seam on our input stream parameter. And our tests still pass.

Let's try substituting a hardcoded string in lieu of the standard input stream: to that effect, we just need to replace `stdin()` with an object of a class implementing the `Read` trait, that would allow for referencing a string. `Cursor` is such a class:

    A Cursor wraps another type and provides it with a Seek implementation.

    Cursors are typically used with in-memory buffers to allow them to implement Read and/or Write, allowing these buffers to be used anywhere you might use a reader or writer that does actual I/O.

    The standard library implements some I/O traits on various types which are commonly used as a buffer, like Cursor<Vec<u8>> and Cursor<&[u8]>. 

Can we use a Cursor on a hardcoded string ? Let's try:

    use std::io:: { stdin, BufRead, BufReader };

    fn main() {
        process(&mut BufReader::new(Cursor::new("999999\n42\n")))
    }

    fn process<T:BufRead>(input : &mut T) {
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

And now running the tests 

    ./test_script ⏎

    0a1
    > 999999

    1c1
    < 4807
    ---
    > 999999

reveal that the first diff found 999999 as a surplus, and the second one, that 999999 came up where 4807 was expected.  

It worked! We can now command the seam to represent either a hardcoded string, or the standard input buffer. 

### 8. Creating a seam on the output stream
   
Creating the seam for the output stream is a bit more indirect, as the `print!` macro makes this output stream implicit. Let's make it explicit by replacing `print!` with it's `write!` equivalent.

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

            write!(stdout(), "{}", buffer)
                .expect("output error");

            if buffer == "42\n" {
                break
            }
        }
    }

Now we make `stdout()` a parameter value for the `process` function. The documentation says that this macro is typically used with a buffer of `&mut Write` so let's use that:

    fn main() {
        process(&mut BufReader::new(stdin()),
                &mut stdout());
    }

    fn process<T:BufRead>(input : &mut T, output : &mut Write) {
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

### Substituting a buffer to stdout()

Now we can use a `Cursor` initialized with a `vec` in the place of  `stdout()`, 

    // http://www.spoj.com/problems/EXPECT/
    use std::io:: { stdin, BufRead, BufReader, 
                    stdout, Write, Cursor };

    fn main() {
        process(&mut BufReader::new(stdin()),
                &mut Cursor::new(vec!()));
    }

And now, running the test reveals that the program didn't print anything:

1d0
< 42

1,2d0
< 4807
< 42

since the output is now written in a Cursor instead of the standard output.


### Writing automated test with the seams

Now that we know we can manipulate seams, we can use them for testing. The idea is to :

1. set up an input buffer with a hard coded string, e.g. `"4807\n42\n"`
2. set up the output buffer to a new `Vec` 
3. call our `process` function with these parameters
4. get the content of the output buffer
5. compare this content with an expected result (here `"4807\n42\n"`) 

Step 4  is done using  `into_inner()` to access the inner representation of the buffer, and `from_utf8` to convert the utf-8 codes into a standard string.


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

### Conclusion

What we have done here:

- creating seams on the input and output streams of our program
- creating commands on these seams to switch them from standard I/O to Cursors
- writing unit tests using our Cursor-based streams as mock

allows us to create more sophisticated programs with the TDD approach. Of course, designing our program only through unit tests on the I/O level is not a panacea: for more complex program, we would need to have tests on the level of our domain objects and functions, not only on I/O. 
