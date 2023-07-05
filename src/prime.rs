use num_integer::Roots;

pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    if n <= 1 {
        return vec![];
    }

    let n: usize = n;
    let mut sieve = vec![true; n + 1];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..=n.sqrt() {
        if !sieve[i] {
            continue;
        }

        let mut j = i * 2;
        while j <= n {
            sieve[j] = false;
            j += i;
        }
    }

    sieve
        .iter()
        .enumerate()
        .filter(|(_, x)| **x)
        .map(|(i, _)| i)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sieve_of_eratosthenes() {
        assert_eq!(Vec::<usize>::new(), sieve_of_eratosthenes(0));
        assert_eq!(Vec::<usize>::new(), sieve_of_eratosthenes(1));
        assert_eq!(vec![2], sieve_of_eratosthenes(2));
        assert_eq!(vec![2, 3, 5, 7], sieve_of_eratosthenes(10));
        assert_eq!(vec![2, 3, 5, 7, 11], sieve_of_eratosthenes(11));
    }
}
