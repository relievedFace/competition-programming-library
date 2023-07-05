use num_traits::Num;
use std::error::Error;

pub fn vec_char_to_num<T>(s: &[char], radix: u32) -> Result<T, Box<dyn Error>>
where
    T: Num,
    <T as Num>::FromStrRadixErr: Error,
    <T as num_traits::Num>::FromStrRadixErr: 'static,
{
    let s: String = s.iter().collect();
    let result = T::from_str_radix(&s, radix)?;
    Ok(result)
}
