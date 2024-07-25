use std::sync::{Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use sp_core::{Pair, sr25519};
use regex::Regex;
use std::time::Instant;
use sp_core::crypto::Ss58Codec;
use crossbeam::channel;
use std::{env, thread};
use ansi_term::Color::{Blue, Red};
use lazy_static::lazy_static;
use term_size::dimensions;

lazy_static! {
    static ref PATTERN: Regex = {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            panic!("Please provide a regex pattern as a command line argument.");
        }
        let pattern = &args[1];
        Regex::new(pattern).expect("Invalid regex pattern")
    };
}

fn print_horizontal_line() {
    if let Some((width, _)) = dimensions() {
        println!("{}", Blue.paint("-".repeat(width)));
    } else {
        println!("{}", Blue.paint("-".repeat(80)));
    }
}

fn highlight_match(input: &str) {
    let re = &*PATTERN;

    let mut last_end = 0;
    for mat in re.find_iter(input) {
        print!("{}", &input[last_end..mat.start()]);
        print!("{}", Red.paint(&input[mat.start()..mat.end()]));
        last_end = mat.end();
    }

    if last_end < input.len() {
        print!("{}", &input[last_end..]);
    }
    println!()
}

fn main() {
    let vanity_pattern = &*PATTERN;
    let start_time = Instant::now();
    let num_threads = num_cpus::get();
    let (sender, receiver) = channel::unbounded();

    let counter = Arc::new(AtomicUsize::new(0));

    let counter_clone = Arc::clone(&counter);
    for _ in 0..num_threads {
        let vanity_pattern = vanity_pattern.clone();
        let sender = sender.clone();
        let counter = Arc::clone(&counter);

        thread::spawn(move || {
            loop {
                let (pair, seed) = sr25519::Pair::generate();
                let public = pair.public();
                let address = public.to_ss58check();
                counter.fetch_add(1, Ordering::SeqCst);

                if vanity_pattern.is_match(&address) {
                    println!("Generated: {:?} addresses", counter.load(Ordering::SeqCst));
                    sender.send((address, seed)).unwrap();
                }
            }
        });
    }

    ctrlc::set_handler(move || {
        println!("Program interrupted! Exiting after generating {:?} addresses in {:.2?}...", counter_clone.load(Ordering::SeqCst), start_time.elapsed());
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    loop {
        let (address, seed) = receiver.recv().unwrap();
        let elapsed_time = start_time.elapsed();

        print!("Vanity Address Found: ");
        highlight_match(&*address);
        println!("Secret Seed: 0x{}", hex::encode(seed));
        println!("Time elapsed: {:.2?}", elapsed_time);
        print_horizontal_line();
    }

}
