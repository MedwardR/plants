use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::f32::consts::{PI, TAU};
use std::ops::{Add, AddAssign, Rem, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Angle {
    value: f32,
}

impl Serialize for Angle {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.degrees().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Angle {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let degrees = f32::deserialize(deserializer)?;
        Ok(Self::from_degrees(degrees))
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value - rhs.value,
        }
    }
}

impl Rem for Angle {
    type Output = Angle;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value % rhs.value,
        }
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl Angle {
    pub const ZERO: Angle = Angle { value: 0.0 };
    pub const PI: Angle = Angle { value: PI };
    pub const TAU: Angle = Angle { value: TAU };

    pub const fn from_degrees(degrees: f32) -> Self {
        Self {
            value: degrees.to_radians(),
        }
    }

    pub const fn from_radians(radians: f32) -> Self {
        Self { value: radians }
    }

    pub fn degrees(&self) -> f32 {
        self.value.to_degrees()
    }

    pub fn radians(&self) -> f32 {
        self.value
    }

    pub fn sin(&self) -> f32 {
        self.value.sin()
    }

    pub fn cos(&self) -> f32 {
        self.value.cos()
    }
}
