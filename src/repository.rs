use std::str::FromStr;

pub enum RepositoryType {
    HashMap,
    Sqlite,
}

impl FromStr for RepositoryType {
    type Err = ();

    fn from_str(s: &str) -> Result<RepositoryType, ()> {
        match s {
            "hashmap" => Ok(RepositoryType::HashMap),
            "sqlite" => Ok(RepositoryType::Sqlite),
            _ => Err(()),
        }
    }
}
