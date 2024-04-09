use fake::Dummy;
use fake::{Fake, Faker};
use rand::Rng;

#[derive(Debug)]
pub struct Picsum(String);

impl Picsum {
    pub fn new() -> Self {
        Faker.fake()
    }
}

impl Dummy<Faker> for Picsum {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _: &mut R) -> Self {
        Picsum(format!(
            "https://picsum.photos/seed/{}/500/500",
            10.fake::<String>()
        ))
    }
}

impl ToString for Picsum {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
