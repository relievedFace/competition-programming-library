#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModInt<const P: i64>(i64);

impl<const P: i64> From<i64> for ModInt<{ P }> {
    fn from(val: i64) -> Self {
        ModInt(val % P)
    }
}

impl<const P: i64> std::ops::Add for ModInt<{ P }> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ModInt((self.0 + rhs.0) % P)
    }
}

impl<const P: i64> std::ops::AddAssign for ModInt<P> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<const P: i64> std::ops::Sub for ModInt<P> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self >= rhs {
            ModInt(self.0 - rhs.0)
        } else {
            ModInt(rhs.0 + P - self.0)
        }
    }
}

impl<const P: i64> std::ops::SubAssign for ModInt<P> {
    fn sub_assign(&mut self, rhs: Self) {
        if *self >= rhs {
            self.0 -= rhs.0
        } else {
            self.0 += P - self.0;
        }
    }
}

impl<const P: i64> std::ops::Mul for ModInt<P> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        ModInt((self.0 * rhs.0) % P)
    }
}

impl<const P: i64> std::ops::MulAssign for ModInt<P> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.0 %= P;
    }
}

impl<const P: i64> std::ops::Rem for ModInt<P> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        ModInt(self.0 % rhs.0)
    }
}

impl<const P: i64> std::ops::RemAssign for ModInt<P> {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0
    }
}

impl<const P: i64> std::ops::Div for ModInt<P> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        ModInt(self.0 / rhs.0)
    }
}

impl<const P: i64> std::ops::DivAssign for ModInt<P> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}
impl<const P: i64> ModInt<P> {
    fn pow(self, n: i64) -> ModInt<P> {
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
pub fn combination<const P: i64>(n: ModInt<P>, r: ModInt<P>) -> ModInt<P> {
    let num = (2..=r.0).fold(1.into(), |a: ModInt<P>, i| a * i.into());
    let den = (n.0 - r.0 + 1..=n.0).fold(1.into(), |a: ModInt<P>, i| a * i.into());
    den * num.pow(P - 2)
}
