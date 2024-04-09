
use crate::constant::MAX_ATTEMPT;
use crate::core::{OwnedRepository, RepositoryError, ID};
use crate::link::Link;
use url::{ParseError, Url};

#[derive(Debug)]
pub enum CreateLinkError {
    InvalidUrl(ParseError),
    MaxAttemptExceeded(usize),
    Internal(RepositoryError),
}

#[derive(Debug)]
pub enum GetLinkError {
    NotFound(ID),
    Internal(RepositoryError),
}

pub struct ShortLinkService {
    repository: OwnedRepository<Link>,
}

impl ShortLinkService {
    pub fn new(repository: OwnedRepository<Link>) -> ShortLinkService {
        ShortLinkService { repository }
    }

    pub fn create_short_link(&mut self, url: String) -> Result<Link, CreateLinkError> {
        Url::parse(&url).map_err(CreateLinkError::InvalidUrl)?;

        let mut link = Link::new(url, None);

        for attempt in 1..=MAX_ATTEMPT {
            match self.repository.insert(link.clone()) {
                Ok(_) => {
                    if attempt > 1 {
                        println!("Succeeded to create {} after {} attempt(s).", link, attempt);
                    }

                    return Ok(link);
                }
                Err(RepositoryError::AlreadyExists(_)) => link = link.next(),
                Err(e) => Err(CreateLinkError::Internal(e))?,
            }
        }

        Err(CreateLinkError::MaxAttemptExceeded(MAX_ATTEMPT))
    }

    pub fn get_link_by_id(&mut self, id: &String) -> Result<Link, GetLinkError> {
        match self.repository.get(&id) {
            Ok(link) => Ok(link),
            Err(RepositoryError::NotFound(id)) => Err(GetLinkError::NotFound(id)),
            Err(e) => Err(GetLinkError::Internal(e)),
        }
    }
}
