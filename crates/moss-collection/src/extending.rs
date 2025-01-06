use std::hash::Hash;

pub trait Extend<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>;
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
