
fn main() {
    use std::io;

    fn print_pattern(rows : u32, cols : u32, size : u32) {
        for row in 0..rows*(size+1)+1 {
            for col in 0..cols*(size+1)+1 {
                if (row % (size+1) == 0) || (col % (size+1) == 0)  { 
                    print!("*")
                }
                else {
                    let squ_row = row/(size+1);
                    let squ_col = col/(size+1);
                    let rel_row = row%(size+1)-1;
                    let rel_col = col%(size+1)-1;
                    if (squ_row + squ_col)%2 == 0 {
                        if rel_row == rel_col {
                            print!("\\")
                        }
                        else {
                            print!(".")
                        }
                    }
                    else {
                        if (rel_row) == (size-1-rel_col) {
                            print!("/")
                        }
                        else {
                            print!(".")
                        }
                    }
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
                print_pattern(inputs[0], inputs[1],inputs[2]);
                println!("")
            }
            Err(_) => {
                break
            }
        }
    }
}
