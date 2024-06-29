use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    INT(i32),
    CHAR(u8),
}

impl Value {
    pub fn add(self, rhs: Self) -> Option<Self> {
        match (self, rhs) {
            (Value::INT(lhs), Value::INT(rhs)) => Some(Value::INT(lhs + rhs)),
            _ => None,
        }
    }

    pub fn sub(self, rhs: Self) -> Option<Self> {
        match (self, rhs) {
            (Value::INT(lhs), Value::INT(rhs)) => Some(Value::INT(lhs - rhs)),
            (Value::CHAR(lhs), Value::CHAR(rhs)) => Some(Value::INT(lhs as i32 - rhs as i32)),
            _ => None,
        }
    }
}

impl PartialEq<i32> for Value {
    fn eq(&self, rhs: &i32) -> bool {
        match self {
            Value::INT(lhs) => *lhs == *rhs,
            Value::CHAR(_) => false,
        }
    }
}

impl PartialOrd<i32> for Value {
    fn partial_cmp(&self, rhs: &i32) -> Option<Ordering> {
        match self {
            Value::INT(lhs) => lhs.partial_cmp(rhs),
            Value::CHAR(_) => None,
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs).expect("Cannot add INT & CHAR")
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub(rhs).expect("Cannot sub INT & CHAR")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // region:add
    #[test]
    fn add_ints() {
        let a = Value::INT(-5);
        let b = Value::INT(10);
        assert_eq!(Value::INT(5), a.add(b).unwrap());
    }

    #[test]
    fn add_ints_trait() {
        let a = Value::INT(-5);
        let b = Value::INT(10);
        assert_eq!(Value::INT(5), a + b);
    }

    #[test]
    fn add_chars() {
        let a = Value::CHAR(1);
        let b = Value::CHAR(2);
        assert_eq!(None, a.add(b));
    }

    #[test]
    #[should_panic]
    fn add_chars_trait() {
        let _ = Value::CHAR(1) + Value::CHAR(2);
    }

    #[test]
    fn add_mixed() {
        let a = Value::INT(0);
        let b = Value::CHAR(0);
        assert_eq!(None, a.add(b));

        let a = Value::INT(0);
        let b = Value::CHAR(0);
        assert_eq!(None, b.add(a));
    }

    #[test]
    #[should_panic]
    fn add_mixed_trait() {
        let _ = Value::INT(0) + Value::CHAR(0);
    }
    // endregion

    // region:sub
    #[test]
    fn sub_ints() {
        let a = Value::INT(-5);
        let b = Value::INT(10);
        assert_eq!(Value::INT(-15), a.sub(b).unwrap());
    }

    #[test]
    fn sub_ints_trait() {
        let a = Value::INT(-5);
        let b = Value::INT(10);
        assert_eq!(Value::INT(-15), a - b);
    }

    #[test]
    fn sub_chars() {
        let a = Value::CHAR(1);
        let b = Value::CHAR(2);
        assert_eq!(Value::INT(-1), a.sub(b).unwrap());
    }

    #[test]
    fn sub_chars_trait() {
        let a = Value::CHAR(1);
        let b = Value::CHAR(2);
        assert_eq!(Value::INT(-1), a - b);
    }

    #[test]
    fn sub_mixed() {
        let a = Value::INT(0);
        let b = Value::CHAR(0);
        assert_eq!(None, a.sub(b));

        let a = Value::INT(0);
        let b = Value::CHAR(0);
        assert_eq!(None, b.sub(a));
    }

    #[test]
    #[should_panic]
    fn sub_mixed_trait() {
        let _ = Value::INT(0) + Value::CHAR(0);
    }
    // endregion

    // region:cmp
    #[test]
    fn compare_int() {
        let value = Value::INT(0);
        assert_eq!(value, 0);
        assert!(!(value < 0));
        assert!(value <= 0);
        assert!(!(value > 0));
        assert!(value >= 0);
    }

    #[test]
    fn compare_char() {
        let value = Value::CHAR(0);
        assert!(!(value == 0));
        assert!(!(value < 0));
        assert!(!(value <= 0));
        assert!(!(value > 0));
        assert!(!(value >= 0));
    }
    // endregion
}
