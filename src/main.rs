use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use tiny_url::constant::ITERATIONS;
use tiny_url::constant::THREADS;
use tiny_url::core::Entity;
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
                let url = i.to_string();
                let result = use_case.lock().unwrap().create_short_link(url.clone());
                match result.map(|link| link.get_id()) {
                    Ok(id) => {
                        println!("Thread {}: generated {}", i, id);
                    }
                    Err(err) => {
                        println!("Thread {}: generate failed: {:?}", i, err);
                    }
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
