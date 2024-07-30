use hashbrown::HashSet;
use std::hash::Hash;

pub trait MaybeExtend<T> {
    fn maybe_extend<I>(&mut self, option: Option<I>)
    where
        I: IntoIterator<Item = T>;
}

impl<T> MaybeExtend<T> for HashSet<T>
where
    T: Eq + Hash,
{
    fn maybe_extend<I>(&mut self, option: Option<I>)
    where
        I: IntoIterator<Item = T>,
    {
        if let Some(iterable) = option {
            self.extend(iterable);
        }
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
