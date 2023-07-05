use num_traits::Num;
use std::error::Error;

/// charの配列を数値に変換。
/// 基数を引数のradixで指定。
pub fn from_vec_char_radix<T>(s: &[char], radix: u32) -> Result<T, Box<dyn Error>>
where
    T: Num,
    <T as Num>::FromStrRadixErr: Error,
    <T as Num>::FromStrRadixErr: 'static,
{
    let s: String = s.iter().collect();
    let result = T::from_str_radix(&s, radix)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_char_to_num() {
        let s: Vec<char> = "111".chars().collect();
        assert_eq!(7i32, from_vec_char_radix::<i32>(&s, 2).unwrap());

        let s: Vec<char> = "111".chars().collect();
        assert_eq!(111i32, from_vec_char_radix::<i32>(&s, 10).unwrap());

        let s: Vec<char> = "2000".chars().collect();
        assert_eq!(2000usize, from_vec_char_radix::<usize>(&s, 10).unwrap());
    }

    #[test]
    #[should_panic]
    fn panic_test_vec_char_to_num() {
        let s: Vec<char> = "b2000".chars().collect();
        let _result = from_vec_char_radix::<usize>(&s, 10).unwrap();
    }
}
