const PRIME_NUMBER: i64 = 1_000_000_007;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModInt(i64);

impl Into<ModInt> for i64 {
    fn into(self) -> ModInt {
        ModInt(self)
    }
}

impl std::ops::Add for ModInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ModInt((self.0 + rhs.0) % PRIME_NUMBER)
    }
}

impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub for ModInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self >= rhs {
            ModInt(self.0 - rhs.0)
        } else {
            ModInt(rhs.0 + PRIME_NUMBER - self.0)
        }
    }
}

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        if *self >= rhs {
            self.0 -= rhs.0
        } else {
            self.0 += PRIME_NUMBER - self.0;
        }
    }
}

impl std::ops::Mul for ModInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        ModInt((self.0 * rhs.0) % PRIME_NUMBER)
    }
}

impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.0 %= PRIME_NUMBER;
    }
}

impl std::ops::Rem for ModInt {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        ModInt(self.0 % rhs.0)
    }
}

impl std::ops::RemAssign for ModInt {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0
    }
}

impl std::ops::Div for ModInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        ModInt(self.0 / rhs.0)
    }
}

impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}
impl ModInt {
    fn pow(self, n: i64) -> ModInt {
        if n == 0 {
            1.into()
        } else if n % 2 == 0 {
            let d = self.pow(n / 2);
            d * d
        } else {
            self * self.pow(n - 1)
        }
    }
}
pub fn combination(n: ModInt, r: ModInt) -> ModInt {
    let num = (2..=r.0).fold(1.into(), |a: ModInt, i| a * i.into());
    let den = (n.0 - r.0 + 1..=n.0).fold(1.into(), |a: ModInt, i| a * i.into());
    den * num.pow(PRIME_NUMBER - 2)
}
