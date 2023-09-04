use clap::Parser;
use sha2::{Sha256, Digest};
use std::collections::BTreeMap;
use rayon::prelude::*;

#[derive(Parser, Debug)]
struct Config {
    #[arg(short = 'N', long, default_value_t = 0)]
    nulls: usize,
    #[arg(short = 'F', long, default_value_t = 3)]
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

    let cpus = num_cpus::get();
    let (task_sender, task_receiver) = crossbeam::channel::unbounded();
    let (result_sender, result_receiver) = crossbeam::channel::unbounded();

    let work_range = 1..cpus;
    for i in work_range.clone() {
        task_sender.send(Some(i)).unwrap();
    }
    let mut i = cpus;

    let receivers: Vec<_> = work_range.map(|_| (task_receiver.clone(), result_sender.clone())).collect();
    drop(task_receiver);
    drop(result_sender);

    rayon::join(move || {
        let mut found = BTreeMap::new();
            while let Ok(result) = result_receiver.recv() {
                if let Some((key, value)) = result {
                    found.insert(key, value);
                }
                if found.len() >= args.find {
                    task_sender.send(None).unwrap();
                } else {
                    task_sender.send(Some(i)).unwrap();
                    i += 1;
                }
            }
            drop(result_receiver);

            for (i, (number, hash)) in found.into_iter().enumerate() {
                if i < args.find {
                    println!("{}, \"{}\"", number, hex::encode(hash));
                } else {
                    break;
                }
            }
        },
        move ||
        receivers.par_iter().for_each(|(task, result)| {
            while let Ok(Some(i)) = task.recv() {
                let mut hasher = Sha256::new();
                hasher.update(i.to_string().into_bytes());
                let hash = hasher.finalize();

                if check_zeros(&hash, args.nulls) {
                    if let Err(e) = result.send(Some((i, hash))) {
                        println!("{}", e);
                    }
                } else if let Err(e) = result.send(None) {
                    println!("{}", e);
                }
            }
        })
    );
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