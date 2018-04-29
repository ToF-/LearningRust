// 0         1         2         3         4         5         7
// 0123456789012345678901234567890123456789012345678901234567890123
// XSBSRasdew9873465hkldsfsalndfvnfq489uqovkLKJHaeDaae555Sk5asdpASD
// 55 : k
// 46 : e

#[cfg(test)]
mod tests {
}

fn main() {
    use std::io;
    use std::borrow::Cow;
    const MAX_PWD:usize = 200;
    fn bit(n : u8, b:u8) -> u8 {
        b & (1 << n)
    }
    fn bit_shift3(n : u8, b:u8) -> u8 {
        let p = (n+3)%6;
        ((b & (1 << p)) >> p) << n
    }

    fn key(s : &str, f: &Fn(u8, u8) -> u8 ) -> u8 {
        let bytes = s.as_bytes();
        (0..6).fold(0, |sum, i:usize| sum | f(i as u8,bytes[i]))
    }
    
    fn key_a(s : &str) -> u8 {
        key(s, &bit)
    }
    fn key_b(s : &str) -> u8 {
        key(s, &bit_shift3)

    }
    fn keys(group : &str) -> (u8,u8) {
        (key_a(group),key_b(group))
    }
    fn password<'a>(groups:Vec<&str>, line:&str) -> Cow<'a,str> {
        let l = line.as_bytes();
        Cow::Owned(groups.into_iter().map(|group| keys(group) )
                   .fold(String::with_capacity(MAX_PWD), 
                        |mut result:String, (k_a,k_b):(u8,u8) |
            { result.push(l[k_a as usize] as char); 
              result.push(l[k_b as usize] as char); result } ))
    }


    #[test]
    fn  bit_can_read_the_nth_bit_of_a_byte() {
        assert_eq!( 1, bit(0,0xff_u8));
        assert_eq!( 2, bit(1,0xff_u8));
        assert_eq!( 0, bit(1,0xf0_u8));
        assert_eq!(32, bit(5,0xf0_u8));
    }
    #[test]
    fn  key_a_can_read_the_key_from_a_string() {
        let s = "qwe345";
        assert_eq!(55, key_a(s));
    }   
    #[test]
    fn  key_b_can_read_the_key_from_a_string() {
        let s = "qwe345";
        assert_eq!(46, key_b(s));
    }   
    #[test]
    fn password_can_read_password_from_strings() {
        let groups = vec!["qwe345","rf3Arg"];
        let line   = "XSBSRasdew9873465hkldsfsalndfvnfq489uqovkLKJHaeDaae555Sk5asdpASD";
        assert_eq!("keep", password(groups,line));
        let groups = vec!["qwe345"];
        let line   = "XSBSRasdew9873465hkldsfsalndfvnfq489uqovkLKJHaeDaae555Sk5asdpASD";
        assert_eq!("ke", password(groups,line));
        let groups = vec!["2S4J5K","111111","lrtb2A"];
        let line   = "isimgsow45ipfgisd56wfgngdfcdkgc7kKKKkuuJJgfstdygQdWORQADFSLKF2K8";
        assert_eq!("coding", password(groups,line));
    }

    fn read_input() -> io::Result<String> {
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;


        Ok(input)
    }
    let max_cases = read_input().expect("no input").trim().parse().expect("not a number");
    for _ in 0..max_cases {
        let max_groups:usize = read_input().expect("no input").trim().parse().expect("not a number");
        let first_line = read_input().expect("no input");
        let second_line= read_input().expect("no input");
        let groups : Vec<&str> = first_line.trim().split(' ').collect();
        assert_eq!(max_groups,groups.len());
        println!("{}", password(groups, &second_line));
        read_input();

        }
}
