pub fn iter_index_and_element<T>(v: &[Vec<T>]) -> impl Iterator<Item = (usize, usize, T)> + '_
where
    T: Clone,
{
    v.iter()
        .enumerate()
        .flat_map(|(i, v)| v.iter().enumerate().map(move |(j, e)| (i, j, e.clone())))
}

pub trait Vec2DUtils<T> {
    fn check_bound_get(&self, px: usize, py: usize, dx: isize, dy: isize) -> Option<&T>;
    fn check_bound_get_mut(&mut self, px: usize, py: usize, dx: isize, dy: isize)
        -> Option<&mut T>;
}

impl<T> Vec2DUtils<T> for Vec<Vec<T>> {
    /// self[py + dy][px + dx]を返す。
    ///
    /// # Examples
    ///
    /// ```
    /// use competitive_programming_library::vec::Vec2DUtils;
    ///
    /// let v = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6],
    ///     vec![7, 8, 9],
    ///     vec![10, 11, 12],
    /// ];
    ///
    /// assert_eq!(v.check_bound_get(2, 2, 2, 2), None);
    /// assert_eq!(v.check_bound_get(1, 1, 1, 1), Some(&9));
    /// ```
    fn check_bound_get(&self, px: usize, py: usize, dx: isize, dy: isize) -> Option<&T> {
        if let (Some(px), Some(py)) = (px.checked_add_signed(dx), py.checked_add_signed(dy)) {
            self.get(py).and_then(|v| v.get(px))
        } else {
            None
        }
    }

    /// self[py + dy][px + dx]を返す。
    ///
    /// # Examples
    ///
    /// ```
    /// use competitive_programming_library::vec::Vec2DUtils;
    ///
    /// let mut v = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6],
    ///     vec![7, 8, 9],
    ///     vec![10, 11, 12],
    /// ];
    ///
    /// let e = v.check_bound_get_mut(1, 1, 1, 1).unwrap();
    /// *e += 1;
    /// assert_eq!(e, &10i32);
    /// ```
    fn check_bound_get_mut(
        &mut self,
        px: usize,
        py: usize,
        dx: isize,
        dy: isize,
    ) -> Option<&mut T> {
        if let (Some(px), Some(py)) = (px.checked_add_signed(dx), py.checked_add_signed(dy)) {
            self.get_mut(py).and_then(|v| v.get_mut(px))
        } else {
            None
        }
    }
}
