//! Utils used in the program.
//!
//!

/// Checks that the given byte slice ends with the given number of nulls in hex representation.
/// ```
/// let is_3_zeros = check_zeros(hex!("abcd3000"), 3);
/// println!("{}", is_3_zeros); //Prints "true"
/// ```
pub fn check_zeros(v: &[u8], target: usize) -> bool {
    let mut zeros = 0;
    for &i in v.iter().rev() {
        if i == 0 {
            zeros += 2;
        } else {
            if i & 0x0f == 0 {
                zeros += 1;
            }
            break;
        }
        if zeros >= target {
            break;
        }
    }
    zeros >= target
}

#[cfg(test)]
mod tests{
    use hex_literal::hex;
    use crate::check_zeros;

    #[test]
    fn checker() {
        let h = hex!("abcd3000");
        assert!(check_zeros(&h, 0));
        assert!(check_zeros(&h, 3));
        assert!(!check_zeros(&h, 5));

        let h = hex!("bbbbbbbb");
        assert!(check_zeros(&h, 0));
        assert!(!check_zeros(&h, 1));
    }
}