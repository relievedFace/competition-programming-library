/// マイナス無限大方向に丸める除算
pub fn floor_inf<T>(x: T, m: T) -> T
where
    T: Clone + Copy + num_integer::Integer,
{
    let r = (x % m + m) % m;
    (x - r) / m
}
