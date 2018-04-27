// 0         1         2         3         4         5         7
// 0123456789012345678901234567890123456789012345678901234567890123
// XSBSRasdew9873465hkldsfsalndfvnfq489uqovkLKJHaeDaae555Sk5asdpASD
// 55 : k
// 46 : e

#[cfg(test)]
mod tests {
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
}
