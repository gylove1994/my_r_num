
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, RemAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub enum Number {
    PositiveInfinity,
    NegativeInfinity,
    NaN,
    Integer64(i64),
    Integer32(i32),
    Integer16(i16),
    Integer8(i8),
    Float64(f64),
    Float32(f32),
}

impl Number {
    pub fn from_int(value: i64) -> Self {
        if value >= i8::MIN as i64 && value <= i8::MAX as i64 {
            Number::Integer8(value as i8)
        } else if value >= i16::MIN as i64 && value <= i16::MAX as i64 {
            Number::Integer16(value as i16)
        } else if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
            Number::Integer32(value as i32)
        } else {
            Number::Integer64(value)
        }
    }
    pub fn from_float(value: f64) -> Self {
        let as_f32 = value as f32;
        if (as_f32 as f64 - value).abs() < f64::EPSILON && value.is_finite() {
            Number::Float32(as_f32)
        } else {
            Number::Float64(value)
        }
    }
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.trim().to_lowercase().as_str() {
            "inf" | "infinity" | "+inf" | "+infinity" => return Ok(Number::PositiveInfinity),
            "-inf" | "-infinity" => return Ok(Number::NegativeInfinity),
            "nan" => return Ok(Number::NaN),
            _ => {}
        }
        if let Ok(value) = s.parse::<i64>() {
            return Ok(Self::from_int(value));
        }
        if let Ok(value) = s.parse::<f64>() {
            return Ok(Self::from_float(value));
        }
        Err(format!("无法解析 '{}' 为数字", s))
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            Number::PositiveInfinity => "PositiveInfinity",
            Number::NegativeInfinity => "NegativeInfinity",
            Number::NaN => "NaN",
            Number::Integer8(_) => "Integer8",
            Number::Integer16(_) => "Integer16",
            Number::Integer32(_) => "Integer32",
            Number::Integer64(_) => "Integer64",
            Number::Float32(_) => "Float32",
            Number::Float64(_) => "Float64",
        }
    }
    pub fn to_f64(&self) -> f64 {
        match self {
            Number::PositiveInfinity => f64::INFINITY,
            Number::NegativeInfinity => f64::NEG_INFINITY,
            Number::NaN => f64::NAN,
            Number::Integer64(v) => *v as f64,
            Number::Integer32(v) => *v as f64,
            Number::Integer16(v) => *v as f64,
            Number::Integer8(v) => *v as f64,
            Number::Float64(v) => *v,
            Number::Float32(v) => *v as f64,
        }
    }
    pub fn is_nan(&self) -> bool {
        match self {
            Number::NaN => true,
            Number::Float64(v) => v.is_nan(),
            Number::Float32(v) => v.is_nan(),
            _ => false,
        }
    }
    pub fn is_infinite(&self) -> bool {
        match self {
            Number::PositiveInfinity | Number::NegativeInfinity => true,
            Number::Float64(v) => v.is_infinite(),
            Number::Float32(v) => v.is_infinite(),
            _ => false,
        }
    }
    pub fn is_finite(&self) -> bool {
        !self.is_nan() && !self.is_infinite()
    }
    pub fn from_f64(value: f64) -> Self {
        if value.is_nan() {
            Number::NaN
        } else if value == f64::INFINITY {
            Number::PositiveInfinity
        } else if value == f64::NEG_INFINITY {
            Number::NegativeInfinity
        } else {
            Number::Float64(value)
        }
    }
}
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::PositiveInfinity => write!(f, "∞"),
            Number::NegativeInfinity => write!(f, "-∞"),
            Number::NaN => write!(f, "NaN"),
            Number::Integer64(v) => write!(f, "{}", v),
            Number::Integer32(v) => write!(f, "{}", v),
            Number::Integer16(v) => write!(f, "{}", v),
            Number::Integer8(v) => write!(f, "{}", v),
            Number::Float64(v) => write!(f, "{}", v),
            Number::Float32(v) => write!(f, "{}", v),
        }
    }
}
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_nan() || other.is_nan() {
            return None;
        }
        let self_f64 = self.to_f64();
        let other_f64 = other.to_f64();
        self_f64.partial_cmp(&other_f64)
    }
}
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        if self.is_nan() || other.is_nan() {
            return false;
        }
        self.to_f64() == other.to_f64()
    }
}
impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            return Number::NaN;
        }
        match (self, rhs) {
            (Number::PositiveInfinity, Number::NegativeInfinity) => Number::NaN,
            (Number::NegativeInfinity, Number::PositiveInfinity) => Number::NaN,
            (Number::PositiveInfinity, _) | (_, Number::PositiveInfinity) => {
                Number::PositiveInfinity
            }
            (Number::NegativeInfinity, _) | (_, Number::NegativeInfinity) => {
                Number::NegativeInfinity
            }
            (Number::Integer8(a), Number::Integer8(b)) => Number::from_int(a as i64 + b as i64),
            (Number::Integer16(a), Number::Integer16(b)) => Number::from_int(a as i64 + b as i64),
            (Number::Integer32(a), Number::Integer32(b)) => Number::from_int(a as i64 + b as i64),
            (Number::Integer64(a), Number::Integer64(b)) => {
                if let Some(result) = a.checked_add(b) {
                    Number::Integer64(result)
                } else {
                    Number::Float64(a as f64 + b as f64)
                }
            }
            (
                a @ (Number::Integer8(_)
                | Number::Integer16(_)
                | Number::Integer32(_)
                | Number::Integer64(_)),
                b @ (Number::Integer8(_)
                | Number::Integer16(_)
                | Number::Integer32(_)
                | Number::Integer64(_)),
            ) => {
                let a_val = match a {
                    Number::Integer8(v) => v as i64,
                    Number::Integer16(v) => v as i64,
                    Number::Integer32(v) => v as i64,
                    Number::Integer64(v) => v,
                    _ => unreachable!(),
                };
                let b_val = match b {
                    Number::Integer8(v) => v as i64,
                    Number::Integer16(v) => v as i64,
                    Number::Integer32(v) => v as i64,
                    Number::Integer64(v) => v,
                    _ => unreachable!(),
                };
                if let Some(result) = a_val.checked_add(b_val) {
                    Number::from_int(result)
                } else {
                    Number::Float64(a_val as f64 + b_val as f64)
                }
            }
            _ => {
                let result = self.to_f64() + rhs.to_f64();
                Number::from_float(result)
            }
        }
    }
}
impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            return Number::NaN;
        }
        match (self, rhs) {
            (Number::PositiveInfinity, Number::PositiveInfinity) => Number::NaN,
            (Number::NegativeInfinity, Number::NegativeInfinity) => Number::NaN,
            (Number::PositiveInfinity, _) => Number::PositiveInfinity,
            (Number::NegativeInfinity, _) => Number::NegativeInfinity,
            (_, Number::PositiveInfinity) => Number::NegativeInfinity,
            (_, Number::NegativeInfinity) => Number::PositiveInfinity,
            (
                a @ (Number::Integer8(_)
                | Number::Integer16(_)
                | Number::Integer32(_)
                | Number::Integer64(_)),
                b @ (Number::Integer8(_)
                | Number::Integer16(_)
                | Number::Integer32(_)
                | Number::Integer64(_)),
            ) => {
                let a_val = match a {
                    Number::Integer8(v) => v as i64,
                    Number::Integer16(v) => v as i64,
                    Number::Integer32(v) => v as i64,
                    Number::Integer64(v) => v,
                    _ => unreachable!(),
                };
                let b_val = match b {
                    Number::Integer8(v) => v as i64,
                    Number::Integer16(v) => v as i64,
                    Number::Integer32(v) => v as i64,
                    Number::Integer64(v) => v,
                    _ => unreachable!(),
                };
                if let Some(result) = a_val.checked_sub(b_val) {
                    Number::from_int(result)
                } else {
                    Number::Float64(a_val as f64 - b_val as f64)
                }
            }
            _ => {
                let result = self.to_f64() - rhs.to_f64();
                Number::from_float(result)
            }
        }
    }
}
impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            return Number::NaN;
        }
        let self_f64 = self.to_f64();
        let rhs_f64 = rhs.to_f64();
        if (self.is_infinite() && rhs_f64 == 0.0) || (rhs.is_infinite() && self_f64 == 0.0) {
            return Number::NaN;
        }
        if self.is_infinite() || rhs.is_infinite() {
            let result = self_f64 * rhs_f64;
            return Number::from_f64(result);
        }
        match (self, rhs) {
            (
                a @ (Number::Integer8(_)
                | Number::Integer16(_)
                | Number::Integer32(_)
                | Number::Integer64(_)),
                b @ (Number::Integer8(_)
                | Number::Integer16(_)
                | Number::Integer32(_)
                | Number::Integer64(_)),
            ) => {
                let a_val = match a {
                    Number::Integer8(v) => v as i64,
                    Number::Integer16(v) => v as i64,
                    Number::Integer32(v) => v as i64,
                    Number::Integer64(v) => v,
                    _ => unreachable!(),
                };
                let b_val = match b {
                    Number::Integer8(v) => v as i64,
                    Number::Integer16(v) => v as i64,
                    Number::Integer32(v) => v as i64,
                    Number::Integer64(v) => v,
                    _ => unreachable!(),
                };
                if let Some(result) = a_val.checked_mul(b_val) {
                    Number::from_int(result)
                } else {
                    Number::Float64(a_val as f64 * b_val as f64)
                }
            }
            _ => {
                let result = self_f64 * rhs_f64;
                Number::from_float(result)
            }
        }
    }
}
impl Div for Number {
    type Output = Number;
    fn div(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            return Number::NaN;
        }
        let self_f64 = self.to_f64();
        let rhs_f64 = rhs.to_f64();
        if rhs_f64 == 0.0 && self_f64 == 0.0 {
            return Number::NaN;
        }
        if self.is_infinite() && rhs.is_infinite() {
            return Number::NaN;
        }
        match (self, rhs) {
            (
                a @ (Number::Integer8(_)
                | Number::Integer16(_)
                | Number::Integer32(_)
                | Number::Integer64(_)),
                b @ (Number::Integer8(_)
                | Number::Integer16(_)
                | Number::Integer32(_)
                | Number::Integer64(_)),
            ) => {
                let a_val = match a {
                    Number::Integer8(v) => v as i64,
                    Number::Integer16(v) => v as i64,
                    Number::Integer32(v) => v as i64,
                    Number::Integer64(v) => v,
                    _ => unreachable!(),
                };
                let b_val = match b {
                    Number::Integer8(v) => v as i64,
                    Number::Integer16(v) => v as i64,
                    Number::Integer32(v) => v as i64,
                    Number::Integer64(v) => v,
                    _ => unreachable!(),
                };
                if b_val != 0 && a_val % b_val == 0 {
                    Number::from_int(a_val / b_val)
                } else {
                    let result = a_val as f64 / b_val as f64;
                    Number::from_float(result)
                }
            }
            _ => {
                let result = self_f64 / rhs_f64;
                Number::from_float(result)
            }
        }
    }
}
impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl SubAssign for Number {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl MulAssign for Number {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl DivAssign for Number {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl RemAssign for Number {
    fn rem_assign(&mut self, rhs: Self) {
        if self.is_nan() || rhs.is_nan() {
            *self = Number::NaN;
            return;
        }
        if self.is_infinite() || rhs.is_infinite() {
            *self = Number::NaN;
            return;
        }
        match (&*self, &rhs) {
            (Number::Integer8(a), Number::Integer8(b)) => {
                if *b == 0 {
                    *self = Number::NaN;
                } else {
                    *self = Number::from_int((*a as i64) % (*b as i64));
                }
            }
            (Number::Integer16(a), Number::Integer16(b)) => {
                if *b == 0 {
                    *self = Number::NaN;
                } else {
                    *self = Number::from_int((*a as i64) % (*b as i64));
                }
            }
            (Number::Integer32(a), Number::Integer32(b)) => {
                if *b == 0 {
                    *self = Number::NaN;
                } else {
                    *self = Number::from_int((*a as i64) % (*b as i64));
                }
            }
            (Number::Integer64(a), Number::Integer64(b)) => {
                if *b == 0 {
                    *self = Number::NaN;
                } else {
                    *self = Number::from_int(*a % *b);
                }
            }
            _ => {
                let a_f = self.to_f64();
                let b_f = rhs.to_f64();
                if b_f == 0.0 {
                    *self = Number::NaN;
                } else {
                    *self = Number::from_float(a_f % b_f);
                }
            }
        }
    }
}
// 泛型From实现
impl From<i8> for Number {
    fn from(value: i8) -> Self {
        Number::Integer8(value)
    }
}
impl From<i16> for Number {
    fn from(value: i16) -> Self {
        Number::from_int(value as i64)
    }
}
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::from_int(value as i64)
    }
}
impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number::from_int(value)
    }
}
impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Number::from_int(value as i64)
    }
}
impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Number::from_int(value as i64)
    }
}
impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number::from_int(value as i64)
    }
}
impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Number::from_int(value as i64)
    }
}
impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Number::from_float(value as f64)
    }
}
impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::from_float(value)
    }
}
