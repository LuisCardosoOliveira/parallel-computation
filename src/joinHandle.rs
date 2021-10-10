use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();
    let chunks = input.chunks((input.len() / worker_count).max(1));
    let mut handles = Vec::new();

    for chunk in chunks {
        let string = chunk.join("");
        // return a HashMap from each thread, the JoinHandle wraps this hashmap
        let handle = thread::spawn(move || {
            let mut map: HashMap<char, usize> = HashMap::new();
            for c in string.chars().filter(|c| c.is_alphabetic()) {
                *map.entry(c.to_ascii_lowercase()).or_default() += 1;
            }
            map
        });
        handles.push(handle);
    }

    // wait for each thread to finish and combine every HashMap into the final result
    for handle in handles {
        let map = handle.join().unwrap();
        for (key, value) in map {
            *result.entry(key).or_default() += value;
        }
    }
    result
}
