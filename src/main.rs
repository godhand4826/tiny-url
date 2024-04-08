use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use tiny_url::constant::ITERATIONS;
use tiny_url::constant::THREADS;
use tiny_url::link::Link;
use tiny_url::use_case::create_short_link;

fn main() {
    let ids: Arc<Mutex<HashMap<String, Link>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];
    for i in 0..THREADS {
        let ids = ids.clone();

        let handle = thread::spawn(move || {
            for _ in 0..ITERATIONS {
                let url = i.to_string();
                if let Err(err) = create_short_link(url, ids.clone()) {
                    println!("Thread {}: generate failed: {:?}", i, err);
                }
            }
            println!("Thread {} finished", i);
        });

        handles.push(handle)
    }

    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    println!("All threads finished");
    println!("Expect links generated: {}", THREADS * ITERATIONS);
    println!("Actual links generated: {}", ids.lock().unwrap().len());
}
