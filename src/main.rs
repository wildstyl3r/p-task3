//! Program to print integers, such that their hashes have suffix of nulls of given length, with
//! corresponding hashes.
//!
#![warn(missing_docs)]
use clap::Parser;
use sha2::{Sha256, Digest};
use std::collections::BTreeMap;
use rayon::prelude::*;

mod utils;
use crate::utils::check_zeros;

mod config;
use crate::config::Args;

/// The main function that makes the work done.
///
/// It utilizes rayon to start worker threads that calculate hashes and manager thread that
/// feeds them as they make progress. Multi-sender-multi-consumer channels from crossbeam are used to
/// simplify synchronization.
fn main() {
    let args = Args::parse();

    let cpus = num_cpus::get();
    // task channel is used to feed workers
    let (task_sender, task_receiver) = crossbeam::channel::unbounded();
    // result channel is used to get results
    let (result_sender, result_receiver) = crossbeam::channel::unbounded();

    // work range includes (cpus - 1) elements to allow manager thread to be active simultaneously
    // with worker threads (in the best case)
    let work_range = 1..cpus;
    for i in work_range.clone() {
        task_sender.send(Some(i)).unwrap();
    }
    let mut i = cpus;

    let receivers: Vec<_> = work_range.map(|_| (task_receiver.clone(), result_sender.clone())).collect();
    drop(task_receiver);
    drop(result_sender);

    rayon::join(
        move || {
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