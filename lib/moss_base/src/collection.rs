use hashbrown::HashSet;

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
