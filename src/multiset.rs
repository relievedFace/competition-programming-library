use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Multiset<T: Clone + Ord>(BTreeMap<T, usize>);

impl<T: Clone + Ord> Multiset<T> {
    pub fn new() -> Self {
        Multiset(BTreeMap::new())
    }

    pub fn insert(&mut self, e: T) {
        *self.0.entry(e).or_insert(0) += 1;
    }

    pub fn remove(&mut self, e: &T) -> bool {
        if let Some(v) = self.0.get_mut(e) {
            if *v == 1 {
                self.0.remove(e);
            } else {
                *v -= 1;
            }
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn contains(&self, e: &T) -> bool {
        self.0.contains_key(e)
    }

    pub fn first(&self) -> Option<T> {
        self.0.first_key_value().map(|k| k.0.clone())
    }

    pub fn last(&self) -> Option<T> {
        self.0.last_key_value().map(|k| k.0.clone())
    }
}

impl<T: Clone + Ord> Default for Multiset<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiset() {
        let mut multiset = Multiset::new();
        multiset.insert(0);
        multiset.insert(0);
        multiset.insert(1);
        multiset.insert(1);
        multiset.insert(1);
        multiset.insert(2);
        multiset.insert(3);
        multiset.insert(4);
        multiset.insert(4);

        assert!(multiset.contains(&3));
        assert!(!multiset.contains(&5));

        multiset.remove(&3);
        assert!(!multiset.contains(&3));

        multiset.remove(&4);
        multiset.remove(&4);
        assert!(!multiset.contains(&4));

        assert_eq!(multiset.first(), Some(0));
        assert_eq!(multiset.last(), Some(2));

        multiset.clear();

        assert_eq!(multiset.first(), None);
        assert_eq!(multiset.last(), None);
    }
}
