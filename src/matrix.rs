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

pub fn shift_right<T>(v: &[Vec<T>], shift: usize) -> Vec<Vec<T>>
where
    T: Clone + Default,
{
    let h = v.len();
    let w = v[0].len();
    let mut shifted_v = vec![vec![T::default(); w]; h];

    for i in 0..h {
        for j in 0..w {
            shifted_v[i][(j + shift) % w] = v[i][j].clone();
        }
    }

    shifted_v
}

pub fn shift_left<T>(v: &[Vec<T>], shift: usize) -> Vec<Vec<T>>
where
    T: Clone + Default,
{
    let h = v.len();
    let w = v[0].len();
    let mut shifted_v = vec![vec![T::default(); w]; h];

    for i in 0..h {
        for j in 0..w {
            shifted_v[i][(w + j - shift) % w] = v[i][j].clone();
        }
    }

    shifted_v
}

pub fn shift_down<T>(v: &[Vec<T>], shift: usize) -> Vec<Vec<T>>
where
    T: Clone + Default,
{
    let h = v.len();
    let w = v[0].len();
    let mut shifted_v = vec![vec![T::default(); w]; h];

    for i in 0..h {
        for j in 0..w {
            shifted_v[(i + shift) % h][j] = v[i][j].clone();
        }
    }

    shifted_v
}

pub fn shift_up<T>(v: &[Vec<T>], shift: usize) -> Vec<Vec<T>>
where
    T: Clone + Default,
{
    let h = v.len();
    let w = v[0].len();
    let mut shifted_v = vec![vec![T::default(); w]; h];

    for i in 0..h {
        for j in 0..w {
            shifted_v[(h + i - shift) % h][j] = v[i][j].clone();
        }
    }

    shifted_v
}
