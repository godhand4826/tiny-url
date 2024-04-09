use crate::base58;
use crate::constant::LINK_ID_LEN;
use crate::core::Entity;
use crate::core::ID;
use chrono::NaiveDateTime;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug, Clone)]
pub struct Link {
    pub id: String, // also used as short link
    pub url: String,
    pub expired_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

impl Link {
    pub fn new(url: String, expired_at: Option<NaiveDateTime>) -> Link {
        let now = chrono::Utc::now().naive_utc();
        Link {
            id: link_id(url.as_str(), now),
            url: url,
            expired_at,
            created_at: now,
        }
    }

    pub fn next(&self) -> Link {
        Link {
            id: next_link_id(&self.id),
            ..self.clone()
        }
    }
}

impl Entity for Link {
    fn get_id(&self) -> ID {
        self.id.clone()
    }
}

impl std::fmt::Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Link{{ id: {}, url: {}, created_at: {}, expired_at: {:?}}}",
            self.id,
            self.url,
            self.created_at,
            self.expired_at.map(|t| t.and_utc().to_rfc3339())
        )
    }
}

fn link_id(url: &str, time: NaiveDateTime) -> String {
    let mut state = DefaultHasher::new();
    url.hash(&mut state);
    time.hash(&mut state);
    let hash = state.finish();

    take_last_n(base58::from_u64(hash), LINK_ID_LEN)
}

fn next_link_id(link_id: &str) -> String {
    take_last_n(base58::from_u64(base58::to_u64(&link_id) + 1), LINK_ID_LEN)
}

fn take_last_n(s: String, n: usize) -> String {
    s.chars()
        .rev()
        .take(n)
        .collect::<String>()
        .chars()
        .rev()
        .collect()
}
