fn main() {
    use std::io;
    fn read_input() -> io::Result<String> {
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;


        Ok(input)
    }
    let max_cases = read_input().expect("no input").trim().parse().expect("not a number");
    for _ in 0..max_cases {
        let s = read_input().expect("no input");
        let inputs:Vec<&str> = s.trim().split(" ").collect();
        let n:u32 = inputs[0].parse().expect("Not an integer!");
        let x:u32 = inputs[1].parse().expect("Not an integer!");
        let y:u32 = inputs[2].parse().expect("Not an integer!");
        for i in 1..n {
            if ((i % x) == 0) && ((i % y)!=0) {
                print!(" {}",i)
            }
        }
        println!();

    }
}
