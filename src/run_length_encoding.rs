#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Run<'a, T> {
    pub value: &'a T,
    pub len: usize,
}

pub fn run_length_encoding<T: Eq + Clone>(v: &Vec<T>) -> Vec<Run<T>> {
    let mut result = vec![];
    let mut len = 1;

    if v.is_empty() {
        result
    } else {
        for sub in v.windows(2) {
            if sub[0] != sub[1] {
                result.push(Run {
                    value: &sub[0],
                    len,
                });
                len = 1;
            } else {
                len += 1;
            }
        }
        result.push(Run {
            value: v.last().unwrap(),
            len,
        });
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all() {
        assert_eq!(
            run_length_encoding(&vec![1, 1, 1]),
            vec![Run { value: &1, len: 3 }]
        );
    }

    #[test]
    fn empty() {
        assert_eq!(run_length_encoding::<&i32>(&vec![]), vec![]);
    }

    #[test]
    fn normal() {
        assert_eq!(
            run_length_encoding(&vec![1, 1, 2, 2, 2, 3, 3, 3, 3]),
            vec![
                Run { value: &1, len: 2 },
                Run { value: &2, len: 3 },
                Run { value: &3, len: 4 }
            ]
        );
    }
}
