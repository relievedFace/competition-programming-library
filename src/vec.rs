pub fn iter_index_and_element<T>(v: &[Vec<T>]) -> impl Iterator<Item = (usize, usize, T)> + '_
where
    T: Clone,
{
    v.iter()
        .enumerate()
        .flat_map(|(i, v)| v.iter().enumerate().map(move |(j, e)| (i, j, e.clone())))
}

/// v[py + dy][px + dx]を返す。
///
/// # Examples
///
/// ```
/// use competitive_programming_library::vec::check_bound_get;
///
/// let v = vec![
///     vec![1, 2, 3],
///     vec![4, 5, 6],
///     vec![7, 8, 9],
///     vec![10, 11, 12],
/// ];
///
/// assert_eq!(check_bound_get(&v, 1, 1, 1, 1), Some(&9));
/// assert_eq!(check_bound_get(&v, 2, 2, 2, 2), None);
/// assert_eq!(check_bound_get(&v, 2, 3, -1, -1), Some(&8));
/// ```
pub fn check_bound_get<T>(v: &[Vec<T>], px: usize, py: usize, dx: isize, dy: isize) -> Option<&T> {
    if let (Some(px), Some(py)) = (px.checked_add_signed(dx), py.checked_add_signed(dy)) {
        v.get(py).and_then(|v| v.get(px))
    } else {
        None
    }
}
