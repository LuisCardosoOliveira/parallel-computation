use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let result = Arc::new(Mutex::new(HashMap::new()));
    let chunks = input.chunks((input.len() / worker_count).max(1));
    let mut handles: Vec<_> = Vec::new();

    for chunk in chunks {
        let string = chunk.join("");
        let result = Arc::clone(&result);
        let handle = thread::spawn(move || {
            let mut map: HashMap<char, usize> = HashMap::new();
            // create a HashMap for this chunk
            for c in string.chars().filter(|c| c.is_alphabetic()) {
                *map.entry(c.to_ascii_lowercase()).or_default() += 1;
            }
            // add the HashMap of this chunk to the HashMap that is wrapped by the Mutex
            let mut result = result.lock().unwrap();
            for (key, value) in map {
                *result.entry(key).or_default() += value;
            }
        });
        handles.push(handle);
    }

    // wait for each thread to finish
    for handle in handles {
        handle.join().unwrap()
    }

    // get the HashMap from the Arc<Mutex<HashMap>>
    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}
