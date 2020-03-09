use super::utils::*;
/// RNum represents a rational number.
///
/// # Examples
///
/// ```
/// use rnmat::rnum::RNum;
/// let n1 = RNum::new(1, -2);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RNum {
    neg_flag: bool,
    nume: u32,
    deno: u32,
}

impl RNum {
    pub fn new(n: i32, d: i32) -> RNum {
        assert!(d != 0, "Error, denominator is zero.");
        let flag = (n < 0 && d > 0) || (n > 0 && d < 0);
        if n == 0 {
            return RNum {
                neg_flag: false,
                nume: 0,
                deno: 1,
            };
        }
        let (mut n, mut d) = (n.abs() as u32, d.abs() as u32);
        let gcd_num = gcd(d, n);
        n /= gcd_num;
        d /= gcd_num;

        RNum {
            neg_flag: flag,
            nume: n,
            deno: d,
        }
    }

    pub fn safe_make(n: i32, d: i32) -> Option<RNum> {
        if d == 0 {
            None
        } else {
            Some(RNum::new(n, d))
        }
    }

    pub fn is_negative(&self) -> bool {
        return self.neg_flag && self.nume != 0;
    }

    pub fn is_positive(&self) -> bool {
        return !self.neg_flag && self.nume != 0;
    }

    pub fn is_zero(&self) -> bool {
        return self.nume == 0;
    }

    pub fn zero() -> RNum {
        return RNum::new(0, 1);
    }

    /// Use gcd() to reduce denominator and numerator.
    fn reduce(&mut self) {
        assert!(self.deno != 0, "Error, denominator is zero.");
        if self.nume == 0 {
            self.neg_flag = false;
            self.deno = 1;
        } else {
            let gcd_num = gcd(self.deno, self.nume);
            if gcd_num > 1 {
                self.deno /= gcd_num;
                self.nume /= gcd_num;
            }
        }
    }
}
// ===============================================================
// TODO
// impl PartialEq, Eq, Ord, ...
// impl Add, Mul, Minus, Div
// ===============================================================

impl PartialEq for RNum {
    fn eq(&self, other: &Self) -> bool {
        get_reduced_pair(self.nume, self.deno) == get_reduced_pair(other.nume, other.deno)
            && self.neg_flag == other.neg_flag
    }
}

impl std::ops::Add for RNum {
    type Output = RNum;
    fn add(self, rhs: Self) -> Self::Output {
        let deno = self.deno * rhs.deno;
        let mut nume = self.nume * rhs.deno;
        if rhs.neg_flag {
            nume -= self.deno * rhs.nume;
        } else {
            nume += self.deno * rhs.nume;
        }
        let flag = self.neg_flag ^ rhs.neg_flag;
        // TODO: consider overflow.
        let mut res = RNum {
            neg_flag: flag,
            nume: nume,
            deno: deno,
        };
        res.reduce();
        res
    }
}

impl std::ops::Sub for RNum {
    type Output = Self;
    fn sub(self, rhs: RNum) -> Self::Output {
        let deno = self.deno * rhs.deno;
        let mut nume = self.nume * rhs.deno;
        if rhs.neg_flag {
            nume += self.deno * rhs.nume;
        } else {
            nume -= self.deno * rhs.nume;
        }
        let flag = !(self.neg_flag ^ rhs.neg_flag);
        // TODO: consider overflow.
        let mut res = RNum {
            neg_flag: flag,
            nume: nume,
            deno: deno,
        };
        res.reduce();
        res
    }
}

impl std::ops::Mul for RNum {
    type Output = Self;
    fn mul(self, rhs: RNum) -> Self::Output {
        let deno = self.deno * rhs.deno;
        let nume = self.nume * rhs.nume;
        let flag = self.neg_flag ^ rhs.neg_flag;
        // TODO: consider overflow.
        let mut res = RNum {
            neg_flag: flag,
            nume: nume,
            deno: deno,
        };
        res.reduce();
        res
    }
}

impl std::ops::Div for RNum {
    type Output = Self;
    fn div(self, rhs: RNum) -> Self::Output {
        let deno = self.deno * rhs.nume;
        let nume = self.nume * rhs.deno;
        // TODO: consider overflow.
        let flag = self.neg_flag ^ rhs.neg_flag;
        // TODO: consider overflow.
        let mut res = RNum {
            neg_flag: flag,
            nume: nume,
            deno: deno,
        };
        res.reduce();
        res
    }
}

impl std::ops::Neg for RNum {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut flag = !self.neg_flag;
        if self.nume == 0 {
            flag = false;
        }
        RNum {
            neg_flag: flag,
            nume: self.nume,
            deno: self.deno,
        }
    }
}

#[cfg(test)]
mod test_rnum {
    use super::*;

    #[test]
    fn test_eq() {
        assert_eq!(RNum::new(1, 2), RNum::new(-1, -2));
        assert_eq!(RNum::new(0, 1), RNum::new(0, 3));
        assert_eq!(RNum::new(4, 2), RNum::new(2, 1));
    }

    #[test]
    fn test_negative() {
        assert_eq!(true, RNum::new(1, -2).is_negative());
        assert_eq!(false, RNum::new(1, 2).is_negative());
        assert_eq!(true, RNum::new(-1, 2).is_negative());
        assert_eq!(false, RNum::new(-1, -2).is_negative());
    }

    #[test]
    fn test_zero() {
        assert_eq!(true, RNum::new(0, 100).is_zero());
        assert_eq!(false, RNum::new(1, 2).is_zero());
    }

    #[test]
    #[should_panic]
    fn test_zero_deno() {
        RNum::new(1, 0);
    }

    #[test]
    fn test_safe_make() {
        assert_eq!(None, RNum::safe_make(0, 0));
        assert_eq!(
            Some(RNum {
                neg_flag: false,
                nume: 1,
                deno: 2
            }),
            RNum::safe_make(1, 2)
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(RNum::new(1, 2), RNum::new(1, 4) + RNum::new(1, 4));
        assert_eq!(RNum::new(0, 2), RNum::new(1, 2) + RNum::new(1, -2));
    }

    #[test]
    fn test_sub() {
        assert_eq!(RNum::new(1, 2), RNum::new(1, 4) - RNum::new(-1, 4));
        assert_eq!(RNum::new(0, 2), RNum::new(1, 2) - RNum::new(1, 2));
    }

    #[test]
    fn test_mul() {
        assert_eq!(RNum::new(1, 4), RNum::new(1, 2) * RNum::new(1, 2));
        assert_eq!(RNum::new(3, 8), RNum::new(1, 2) * RNum::new(3, 4));
        assert_eq!(RNum::new(-1, 4), RNum::new(1, -2) * RNum::new(1, 2));
        assert_eq!(RNum::zero(), RNum::new(0, 10) * RNum::new(1, 2));
    }

    #[test]
    fn test_div() {
        assert_eq!(RNum::new(1, 1), RNum::new(1, 2) / RNum::new(1, 2));
        assert_eq!(RNum::new(-1, 1), RNum::new(1, -2) / RNum::new(1, 2));
        assert_eq!(RNum::zero(), RNum::new(0, 10) / RNum::new(1, 2));
    }

    #[test]
    #[should_panic]
    fn test_panic_div_zero() {
        let _ = RNum::new(1, 2) / RNum::new(0, 1);
    }

    #[test]
    fn test_neg() {
        assert_eq!(RNum::new(0, 1), -RNum::new(0, 1));
        assert_eq!(RNum::new(1, 2), -RNum::new(-1, 2));
    }

    #[test]
    #[ignore]
    fn test_overflow() {
        let p: u32 = (1 << 31) - 1;
        assert_eq!(p, 0x7fffffff);
        let p2 = p << 1; // overflow
        assert_eq!(p2, 0xfffffffe);
        assert_eq!(p2 << 1, 0xfffffffc);
    }
}
