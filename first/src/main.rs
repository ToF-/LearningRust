#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn main() {
    let s = String::from("foo ").trim().to_string();
    println!("{}",s);
}
