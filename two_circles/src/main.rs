#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn main () {
    use std::io;
    use std::num;
    use std::cmp::Ordering;
    fn read_input() -> io::Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }
    let max_cases = read_input().expect("no input").trim().parse().expect("not a number");
    for _ in 0..max_cases {
        let line = read_input().expect("no input");
        let inputs:Vec<&str> = line.trim().split(" ").collect();
        let a:f64 = inputs[2].parse().expect("Not a number!");
        let b:f64 = inputs[5].parse().expect("Not a number!");
        let x1:f64; 
        let y1:f64;
        let r1:f64;
        let x2:f64;
        let y2:f64;
        let r2:f64;
        if a>b {
            x1 = inputs[0].parse().expect("Not a number!");
            y1 = inputs[1].parse().expect("not a number!");
            r1 = a;
            x2 = inputs[3].parse().expect("Not a number!");
            y2 = inputs[4].parse().expect("Not a number!");
            r2 = b;
        } else {
            x2 = inputs[0].parse().expect("Not a number!");
            y2 = inputs[1].parse().expect("not a number!");
            r2 = a;
            x1 = inputs[3].parse().expect("Not a number!");
            y1 = inputs[4].parse().expect("Not a number!");
            r1 = b;
        }
        let d:f64 = ((x1-x2)*(x1-x2)+(y1-y2)*(y1-y2)).sqrt();
        println!("{} {} {}",d,d+r2,r1);
        println!("{}", match (d+r2).partial_cmp(&r1) {
            Some(Less)    => "I",
            Some(Greater) => "O",
            Some(Equal)   => "E",
            None    => "."
            });
        }
}
