use std::collections::HashMap;
use std::mem;
use std::sync::mpsc;
use std::thread;

fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();
    // divide the large problem into smaller problems
    let chunks = input.chunks((input.len() / worker_count).max(1));
    // create a channel and get a tuple with the sender and receiver
    let (sender, receiver) = mpsc::channel();

    for chunk in chunks {
        // collect the data for the current chunk into an owned variable before
        // sending it to the thread.
        let sender = sender.clone();
        let string = chunk.join("");

        thread::spawn(move || {
            // Solve each chunk and send the resulting HashMap down the channel
            let mut map: HashMap<char, usize> = HashMap::new();
            for c in string.chars().filter(|c| c.is_alphabetic()) {
                *map.entry(c.to_ascii_lowercase()).or_default() += 1
            }
            sender.send(map).unwrap();
        });
    }

    // drop the original sender, else the channel will remain open, causing
    // the receiver to infinitely wait
    mem::drop(sender);

    // combine every received HashMap
    for received in receiver {
        for (key, value) in received {
            *result.entry(key).or_default() += value;
        }
    }
    result
}

fn main() {
    println!("Hello, world!");
}
