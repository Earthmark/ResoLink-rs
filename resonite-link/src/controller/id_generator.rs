use rand::random;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct IdGenerator {
    prefix: String,
    id: AtomicU64,
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self {
            prefix: format!("{:X}", random::<u32>()),
            id: AtomicU64::new(0),
        }
    }
}

impl IdGenerator {
    pub fn new_with_prefix(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            id: AtomicU64::new(0),
        }
    }

    pub fn next(&mut self) -> String {
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        format!("RS_REPL_{}_{}", self.prefix, id)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_two_unique() {
        let mut id_gen = IdGenerator::default();
        assert_ne!(id_gen.next(), id_gen.next());
    }
}
