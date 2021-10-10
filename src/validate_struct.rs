//! Valiate struct in serde with `TryFrom`

use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Deserialize)]
struct ValueRangeUnchecked {
    min: i32,
    max: i32,
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "ValueRangeUnchecked")]
pub struct ValueRange {
    min: i32,
    max: i32,
}

impl ValueRange {
    pub fn try_new(min: i32, max: i32) -> Result<Self, String> {
        if min <= max {
            Ok(ValueRange { min, max })
        } else {
            Err("Invalid ValueRange".to_string())
        }
    }

    pub fn min(&self) -> i32 {
        self.min
    }
    pub fn max(&self) -> i32 {
        self.max
    }
}

impl TryFrom<ValueRangeUnchecked> for ValueRange {
    type Error = String; // Use String as error type just for simplicity

    fn try_from(value: ValueRangeUnchecked) -> Result<Self, Self::Error> {
        let ValueRangeUnchecked { min, max } = value;
        Self::try_new(min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::ValueRange;

    #[test]
    fn invalid_range() {
        let res: Result<ValueRange, _> = serde_json::from_str(r#"{"min": 10, "max": 9}"#);
        let err = res.unwrap_err();
        assert_eq!(format!("{}", err), "Invalid ValueRange");
    }

    #[test]
    fn valid_range() {
        let range: ValueRange = serde_json::from_str(r#"{"min": 1, "max": 10}"#).unwrap();

        assert_eq!(range.min(), 1);
        assert_eq!(range.max(), 10);
    }
}
