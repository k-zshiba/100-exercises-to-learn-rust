// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}

impl SaturatingU16 {
    fn new(value: u16) -> SaturatingU16 {
        SaturatingU16 {
            value,
        }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self{
        SaturatingU16 { 
            value: value.into(),
        }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self{
        SaturatingU16 { 
            value: (*value).into(),
        }
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self{
        SaturatingU16 { 
            value,
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self{
        SaturatingU16 { 
            value: *value,
        }
    }
}

impl std::ops::Add for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: Self) -> Self::Output{
        self + rhs.value
    }
}

impl std::ops::Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output{
        self + rhs.value
    }
}

impl std::ops::Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output{
        let sum = self.value.saturating_add(rhs);
        Self::new(sum)
    }
}

impl std::ops::Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output{
        let sum = self.value.saturating_add(*rhs);
        Self::new(sum)
    }
}

//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, value: &u16) -> bool {
        &self.value == value
    }
}