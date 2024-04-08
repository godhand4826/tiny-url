use crate::constant::MAX_ATTEMPT;
use crate::link::Link;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
pub enum CreateLinkError {
    MaxAttemptExceeded(usize),
}

pub fn create_short_link(
    url: String,
    ids: Arc<Mutex<HashMap<String, Link>>>,
) -> Result<Link, CreateLinkError> {
    let mut link = Link::new(url, None);

    for attempt in 1..=MAX_ATTEMPT {
        let mut ids = ids.lock().unwrap();

        if ids.contains_key(&link.id) {
            // linear probing to find the next available link
            link = link.next();
            continue;
        } else {
            ids.insert(link.id.clone(), link.clone());

            if attempt > 1 {
                println!("Succeeded to create {} after {} attempt(s).", link, attempt);
            }

            return Ok(link);
        }
    }

    return Result::Err(CreateLinkError::MaxAttemptExceeded(MAX_ATTEMPT));
}
