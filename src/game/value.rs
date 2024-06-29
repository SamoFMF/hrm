use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Write};
use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Int(i32),
    Char(char),
}

impl Value {
    pub fn add(self, rhs: Self) -> Option<Self> {
        match (self, rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => Some(Value::Int(lhs + rhs)),
            _ => None,
        }
    }

    pub fn sub(self, rhs: Self) -> Option<Self> {
        match (self, rhs) {
            (Value::Int(lhs), Value::Int(rhs)) => Some(Value::Int(lhs - rhs)),
            (Value::Char(lhs), Value::Char(rhs)) => Some(Value::Int(lhs as i32 - rhs as i32)),
            _ => None,
        }
    }
}

impl PartialEq<i32> for Value {
    fn eq(&self, rhs: &i32) -> bool {
        match self {
            Value::Int(lhs) => *lhs == *rhs,
            Value::Char(_) => false,
        }
    }
}

impl PartialOrd<i32> for Value {
    fn partial_cmp(&self, rhs: &i32) -> Option<Ordering> {
        match self {
            Value::Int(lhs) => lhs.partial_cmp(rhs),
            Value::Char(_) => None,
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

impl Into<String> for Value {
    fn into(self) -> String {
        match self {
            Value::Int(val) => val.to_string(),
            Value::Char(val) => val.to_string(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(val) => f.write_str(val.to_string().as_str()),
            Value::Char(val) => f.write_char(*val as char),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let value = Value::Int(5);
        let serialized = serde_json::to_string(&value).unwrap();
        println!("{serialized}");

        let value = Value::Char('A');
        let serialized = serde_json::to_string(&value).unwrap();
        println!("{serialized}");
    }

    #[test]
    fn bar() {
        let value = "[1, 2, \"1\", \"B\"]";
        let deserialized: Vec<Value> = serde_json::from_str(value).unwrap();
        println!("{:?}", deserialized);
    }

    // region:add
    #[test]
    fn add_ints() {
        let a = Value::Int(-5);
        let b = Value::Int(10);
        assert_eq!(Value::Int(5), a.add(b).unwrap());
    }

    #[test]
    fn add_ints_trait() {
        let a = Value::Int(-5);
        let b = Value::Int(10);
        assert_eq!(Value::Int(5), a + b);
    }

    #[test]
    fn add_chars() {
        let a = Value::Char('A');
        let b = Value::Char('B');
        assert_eq!(None, a.add(b));
    }

    #[test]
    #[should_panic]
    fn add_chars_trait() {
        let _ = Value::Char('A') + Value::Char('B');
    }

    #[test]
    fn add_mixed() {
        let a = Value::Int(0);
        let b = Value::Char('0');
        assert_eq!(None, a.add(b));

        let a = Value::Int(0);
        let b = Value::Char('0');
        assert_eq!(None, b.add(a));
    }

    #[test]
    #[should_panic]
    fn add_mixed_trait() {
        let _ = Value::Int(0) + Value::Char('0');
    }
    // endregion

    // region:sub
    #[test]
    fn sub_ints() {
        let a = Value::Int(-5);
        let b = Value::Int(10);
        assert_eq!(Value::Int(-15), a.sub(b).unwrap());
    }

    #[test]
    fn sub_ints_trait() {
        let a = Value::Int(-5);
        let b = Value::Int(10);
        assert_eq!(Value::Int(-15), a - b);
    }

    #[test]
    fn sub_chars() {
        let a = Value::Char('A');
        let b = Value::Char('B');
        assert_eq!(Value::Int(-1), a.sub(b).unwrap());
    }

    #[test]
    fn sub_chars_trait() {
        let a = Value::Char('A');
        let b = Value::Char('B');
        assert_eq!(Value::Int(-1), a - b);
    }

    #[test]
    fn sub_mixed() {
        let a = Value::Int(0);
        let b = Value::Char('0');
        assert_eq!(None, a.sub(b));

        let a = Value::Int(0);
        let b = Value::Char('0');
        assert_eq!(None, b.sub(a));
    }

    #[test]
    #[should_panic]
    fn sub_mixed_trait() {
        let _ = Value::Int(0) + Value::Char('0');
    }
    // endregion

    // region:cmp
    #[test]
    fn compare_int() {
        let value = Value::Int(0);
        assert_eq!(value, 0);
        assert!(!(value < 0));
        assert!(value <= 0);
        assert!(!(value > 0));
        assert!(value >= 0);
    }

    #[test]
    fn compare_char() {
        let value = Value::Char('0');
        assert!(!(value == 0));
        assert!(!(value < 0));
        assert!(!(value <= 0));
        assert!(!(value > 0));
        assert!(!(value >= 0));
    }
    // endregion
}
