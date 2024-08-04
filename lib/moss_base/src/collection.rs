use hashbrown::HashSet;
use std::hash::Hash;

pub trait Extend<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>;
}

impl<K, V> Extend<(K, V)> for radix_trie::Trie<K, V>
where
    K: radix_trie::TrieKey + AsRef<str> + Hash + Eq,
    V: Clone,
{
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        iter.for_each(move |(key, value)| {
            self.insert(key, value);
        });
    }
}

impl<'a, K, V> Extend<(&'a K, &'a V)> for radix_trie::Trie<K, V>
where
    K: radix_trie::TrieKey + AsRef<str> + Hash + Eq + Clone,
    V: Clone,
{
    fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        iter.for_each(move |(key, value)| {
            self.insert(key.clone(), value.clone());
        });
    }
}

pub trait MaybeExtend<T> {
    fn maybe_extend<I>(&mut self, option: Option<I>) -> Option<()>
    where
        I: IntoIterator<Item = T>;
}

impl<T> MaybeExtend<T> for hashbrown::HashSet<T>
where
    T: Eq + Hash,
{
    fn maybe_extend<I>(&mut self, option: Option<I>) -> Option<()>
    where
        I: IntoIterator<Item = T>,
    {
        if let Some(iterable) = option {
            self.extend(iterable);

            return Some(());
        }

        None
    }
}

/// Returns a vector of unique values ​​from the input vector.
pub fn distinct<T: Eq + std::hash::Hash + Clone>(array: Vec<T>) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for element in array {
        if seen.insert(element.clone()) {
            result.push(element);
        }
    }

    result
}
