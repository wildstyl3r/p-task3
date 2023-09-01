use clap::Parser;
use sha2::{Sha256, Digest};
use std::sync::{Arc, Mutex};

#[derive(Parser, Debug)]
struct Config {
    #[arg(short = 'N', long)]
    nulls: usize,
    #[arg(short = 'F', long)]
    find: usize
}

fn check_zeros(v: &[u8], target: usize) -> bool {
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

fn main() {
    let args = Config::parse();

    let mut found = Vec::new();
    let mut i: usize = 1;
    while found.len() < args.find {
        let mut hasher = Sha256::new();
        hasher.update(i.to_string().into_bytes());
        let result = hasher.finalize();

        if check_zeros(&result, args.nulls) {
            found.push((i, result));
        }
        i += 1;
    }

    for (i, v) in found {
        println!("{} {:?}", i, v);
    }
}


#[cfg(test)]
mod tests{
    use hex_literal::hex;
    use crate::check_zeros;

    #[test]
    fn checker() {
        let h = hex!("95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000");
        assert!(check_zeros(&h, 3));
        assert!(!check_zeros(&h, 5));

        let h = hex!("0000000000000000000000000000000000000000000000000000000000000000");
        assert!(check_zeros(&h, 1));
        assert!(check_zeros(&h, 0));

        let h = hex!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
        assert!(!check_zeros(&h, 1));
        assert!(check_zeros(&h, 0));
    }
}