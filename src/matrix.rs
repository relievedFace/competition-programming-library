pub use itertools::iproduct;
use itertools::Itertools;

pub fn rotate<T>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let rows = matrix.len();
    let cols = matrix[0].len();

    iproduct!(0..cols, (0..rows).rev())
        .map(|(new_row, new_col)| matrix[new_col][new_row].clone())
        .chunks(rows)
        .into_iter()
        .map(|chunk| chunk.collect())
        .collect()
}
