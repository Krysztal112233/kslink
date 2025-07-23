use std::hash::{DefaultHasher, Hash, Hasher};

use radix_fmt_ng::radix;

pub trait ShortHash {
    fn into_hash(self) -> u64;

    fn into_short_hash(self) -> String
    where
        Self: Sized,
    {
        radix(self.into_hash(), 62).to_string()
    }
}

impl ShortHash for String {
    fn into_hash(self) -> u64 {
        self.as_str().into_hash()
    }
}

impl ShortHash for &str {
    fn into_hash(self) -> u64 {
        let mut hasher = DefaultHasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
