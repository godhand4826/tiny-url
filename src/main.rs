use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use tiny_url::constant::ITERATIONS;
use tiny_url::constant::THREADS;
use tiny_url::core::OwnedRepository;
use tiny_url::link::Link;
use tiny_url::repository;
use tiny_url::service;

fn main() {
    let repo: OwnedRepository<Link> = Box::new(repository::HashMapRepository::new());
    let link_service = Arc::new(Mutex::new(service::ShortLinkService::new(repo)));

    let mut handles = vec![];
    for i in 0..THREADS {
        let use_case = link_service.clone();

        let handle = thread::spawn(move || {
            for _ in 0..ITERATIONS {
                let result = use_case
                    .lock()
                    .unwrap()
                    .create_short_link(format!("https://www.google.com?q={}", i));

                match result {
                    Ok(link) => println!("Thread {}: generated {}", i, link),
                    Err(err) => println!("Thread {}: generate failed: {:?}", i, err),
                }
            }
            println!("Thread {} finished", i);
        });

        handles.push(handle);
    }

    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    println!("All threads finished");
}
