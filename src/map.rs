use std::{collections::HashMap, hash::Hash};

pub fn convert_vec_to_hashmap<T>(vec: &[T]) -> HashMap<T, usize>
where
    T: Clone + Eq + Hash,
{
    vec.iter().cloned().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    })
}
