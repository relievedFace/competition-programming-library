/// 4方向(上下左右)の移動を表すイテレータを返す。
///
/// # Examples
///
/// ```
/// use competitive_programming_library::iter::move_into_4_directions;
///
/// let mut result = vec![];
/// let x = 10usize;
/// let y = 10usize;
///
/// for (dx, dy) in move_into_4_directions() {
///     let px = x.wrapping_add_signed(dx);
///     let py = y.wrapping_add_signed(dy);
///     result.push((px, py))
/// }
///
/// assert_eq!(result, vec![(10, 11), (11, 10), (10, 9), (9, 10)]);
/// ```
pub fn move_into_4_directions() -> impl Iterator<Item = (isize, isize)> {
    [(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter()
}

/// 8方向(上下左右斜め)の移動を表すイテレータを返す。
///
/// # Examples
///
/// ```
/// use competitive_programming_library::iter::move_into_8_directions;
///
/// let mut result = vec![];
/// let x = 10usize;
/// let y = 10usize;
///
/// for (dx, dy) in move_into_8_directions() {
///     let px = x.wrapping_add_signed(dx);
///     let py = y.wrapping_add_signed(dy);
///     result.push((px, py))
/// }
///
/// assert_eq!(result, vec![(10, 11), (11, 11), (11, 10), (11, 9), (10, 9), (9, 9), (9, 10), (9, 11)]);
/// ```
pub fn move_into_8_directions() -> impl Iterator<Item = (isize, isize)> {
    [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ]
    .into_iter()
}
