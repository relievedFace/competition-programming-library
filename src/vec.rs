pub fn iter_index_and_element<T>(v: &[Vec<T>]) -> impl Iterator<Item = (usize, usize, T)> + '_
where
    T: Clone,
{
    v.iter()
        .enumerate()
        .flat_map(|(i, v)| v.iter().enumerate().map(move |(j, e)| (i, j, e.clone())))
}
