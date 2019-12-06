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

pub trait Reader<T: FromStr, R: Read> {
    fn read(&mut self) -> T;
}

impl<R: Read> Reader<String, R> for Scanner<R> {
    fn read(&mut self) -> String {
        let token: Vec<u8> = get_token(&mut self.reader).collect();
        from_utf8(&token).unwrap().to_string()
    }
}

impl<R: Read> Reader<usize, R> for Scanner<R> {
    fn read(&mut self) -> usize {
        get_token(&mut self.reader).fold(0, |a, x| a * 10 + (x - 48) as usize)
    }
}

impl<R: Read> Reader<i64, R> for Scanner<R> {
    fn read(&mut self) -> i64 {
        let mut token = get_token(&mut self.reader);
        let head = token.next().unwrap();
        if head == 45 {
            -token.fold(0 as i64, |a, x| a * 10 + (x - 48) as i64)
        } else {
            token.fold((head - 48) as i64, |a, x| a * 10 + (x - 48) as i64)
        }
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
