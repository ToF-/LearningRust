#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn main() {
    use std::io;
    fn read_input() -> io::Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    let max_xs = read_input().expect("no input").trim().parse().expect("not a number");

    let max_ys = read_input().expect("no input").trim().parse().expect("not a number");
    let line_xs = read_input().expect("no input");
    let line_ys = read_input().expect("no input");
    let xs: Vec<i64> = Vec::new();
    line_xs.trim().split(" ").collect().for_each(| s | 
            xs.push(s.parse().expect("not a number")) 

        );
    println!("foo");
    

}
