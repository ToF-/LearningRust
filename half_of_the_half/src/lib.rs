#[cfg(test)]
mod tests {
    fn half_of_half(s:& str) -> String {
        let mut result = String::new();
        let mut counter = 0;
        let max = s.len() / 2;
        for c in s.chars() {
            if counter % 2 == 0 {
                result += &c.to_string()
            };
            counter+=1;
            if counter >= max { break }
        };
        result
    }
    #[test]
    fn half_of_half_tests() {
        assert_eq!(half_of_half("A"),"A");
        assert_eq!(half_of_half("AB"),"A");
        assert_eq!(half_of_half("ABC"),"A");
        assert_eq!(half_of_half("ABCD"),"A");
        assert_eq!(half_of_half("your"),"y");
        assert_eq!(half_of_half("progress"),"po");
        assert_eq!(half_of_half("is"),"i");
        assert_eq!(half_of_half("noticeable"),"ntc");

    }
}


