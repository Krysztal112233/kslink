use std::hash::{DefaultHasher, Hash, Hasher};

use radix_fmt::radix_32;

pub trait ShortHash {
    fn into_short_hash(self) -> String;
}

impl ShortHash for String {
    fn into_short_hash(self) -> String {
        self.as_str().into_short_hash()
    }
}

impl ShortHash for &str {
    fn into_short_hash<'a>(self) -> String {
        let mut hasher = DefaultHasher::default();
        self.hash(&mut hasher);
        let hash = hasher.finish();

        radix_32(hash).to_string()
    }
}
