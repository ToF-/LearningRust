//    000111
//    012210
// 02 ../\..
// 01 ./..\.
// 00 /....\
// 10 \..../
// 11 .\../.
// 12 ..\/..

fn main() {
    use std::io;

    fn print_pattern(rows:u32, cols:u32, size:u32) {
        for row in 0..rows*size*2 {
            for col in 0..cols*size*2 {
                let mut rel_col : u32;
                let mut rel_row : u32;
                let col_half = (col/size)%2;
                if col_half > 0 {
                    rel_col = (size-1)-(col%size)
                }
                else {
                    rel_col = col%size
                }
                let row_half = (row/size)%2;
                if row_half > 0 {
                    rel_row = row%size
                }
                else {
                    rel_row = (size-1)-(row%size)
                }
                if rel_col == rel_row {
                    if col_half == row_half {
                        print!("/")
                    }
                    else {
                        print!("\\")
                    }
                }
                else {
                    print!(".")
                }
            }
            println!()
        }
    }
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
                print_pattern(inputs[0], inputs[1], inputs[2]);
                println!("")
            }
            Err(_) => {
                break
            }
        }
    }
}
