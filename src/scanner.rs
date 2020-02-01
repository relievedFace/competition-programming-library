use std::io::prelude::*;
use std::str::{from_utf8, FromStr};

pub struct Scanner<R: Read> {
    reader: R,
}

impl<R: Read> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Scanner { reader }
    }
}

pub trait Parse<T: FromStr, R: Read> {
    fn read(&mut self) -> T;
}

impl<R: Read> Parse<String, R> for Scanner<R> {
    fn read(&mut self) -> String {
        let token: Vec<u8> = get_token(&mut self.reader).collect();
        from_utf8(&token).unwrap().to_string()
    }
}

impl<R: Read> Parse<usize, R> for Scanner<R> {
    fn read(&mut self) -> usize {
        let mut token = get_token(&mut self.reader);
        parse_usize(&mut token)
    }
}

impl<R: Read> Parse<i64, R> for Scanner<R> {
    fn read(&mut self) -> i64 {
        let mut token = get_token(&mut self.reader);
        parse_i64(&mut token)
    }
}

fn get_token<'a, R: Read>(reader: &'a mut R) -> impl Iterator<Item = u8> + 'a {
    reader
        .by_ref()
        .bytes()
        .flatten()
        .skip_while(|b| b.is_ascii_whitespace())
        .take_while(|b| !b.is_ascii_whitespace())
}

fn parse_i64<I: Iterator<Item = u8>>(token: &mut I) -> i64 {
    let head = token.next().unwrap();
    if head == 45 {
        -token.fold(0 as i64, |a, x| a * 10 + (x - 48) as i64)
    } else {
        token.fold((head - 48) as i64, |a, x| a * 10 + (x - 48) as i64)
    }
}

fn parse_usize<I: Iterator<Item = u8>>(token: &mut I) -> usize {
    token.fold(0, |a, x| a * 10 + (x - 48) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_i64_test1() {
        let token = "0".as_bytes().to_vec();
        let mut token = token.into_iter();
        assert_eq!(parse_i64(&mut token), 0);
    }

    #[test]
    fn parse_i64_test2() {
        let token = "123456789".as_bytes().to_vec();
        let mut token = token.into_iter();
        assert_eq!(parse_i64(&mut token), 123456789);
    }

    #[test]
    fn parse_i64_test3() {
        let token = "-123456789".as_bytes().to_vec();
        let mut token = token.into_iter();
        assert_eq!(parse_i64(&mut token), -123456789);
    }

    #[test]
    fn parse_usize_test1() {
        let token = "0".as_bytes().to_vec();
        let mut token = token.into_iter();
        assert_eq!(parse_i64(&mut token), 0);
    }

    #[test]
    fn parse_usize_test2() {
        let token = "123456789".as_bytes().to_vec();
        let mut token = token.into_iter();
        assert_eq!(parse_i64(&mut token), 123456789);
    }
}
