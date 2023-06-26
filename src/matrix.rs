use itertools::{iproduct, Itertools};
use std::collections::HashSet;

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

pub fn get_element_bounds<T>(table: &[Vec<T>], e: T) -> Option<[(usize, usize); 4]>
where
    T: Eq,
{
    let mut set = HashSet::new();

    for (y, line) in table.iter().enumerate() {
        for (x, element) in line.iter().enumerate() {
            if *element == e {
                set.insert((y, x));
            }
        }
    }

    if set.is_empty() {
        None
    } else {
        let x1 = *set.iter().map(|(_, x)| x).min().unwrap();
        let y1 = *set.iter().map(|(y, _)| y).min().unwrap();
        let x2 = *set.iter().map(|(_, x)| x).max().unwrap();
        let y2 = *set.iter().map(|(y, _)| y).max().unwrap();

        Some([(x1, y1), (x2, y1), (x2, y2), (x1, y2)])
    }
}
