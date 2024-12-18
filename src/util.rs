use std::{fmt::Display, ops::Add};

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
pub struct U2d(pub usize, pub usize);

impl Add<I2d> for U2d {
    type Output = Result<Self, ()>;
    fn add(self, rhs: I2d) -> Self::Output {
        Ok(Self(
            self.0.checked_add_signed(rhs.0).ok_or(())?,
            self.1.checked_add_signed(rhs.1).ok_or(())?,
        ))
    }
}

impl Add for U2d {
    type Output = Result<Self, ()>;
    fn add(self, rhs: Self) -> Self::Output {
        Ok(Self(
            self.0.checked_add(rhs.0).ok_or(())?,
            self.1.checked_add(rhs.1).ok_or(())?,
        ))
    }
}

impl From<(usize, usize)> for U2d {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0, value.1)
    }
}

impl Display for U2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
pub struct I2d(pub isize, pub isize);

impl Add for I2d {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl From<(isize, isize)> for I2d {
    fn from(value: (isize, isize)) -> Self {
        Self(value.0, value.1)
    }
}

impl TryFrom<u8> for I2d {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'<' => Ok(Self(-1, 0)),
            b'>' => Ok(Self(1, 0)),
            b'^' => Ok(Self(0, -1)),
            b'v' => Ok(Self(0, 1)),
            c => Err(format!("invalid character: {}", c as char)),
        }
    }
}

impl Display for I2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub const DIRECTIONS_4: [I2d; 4] = [I2d(1, 0), I2d(-1, 0), I2d(0, 1), I2d(0, -1)];
pub const DIRECTIONS_8: [I2d; 8] = [
    I2d(1, 0),
    I2d(-1, 0),
    I2d(0, 1),
    I2d(0, -1),
    I2d(-1, -1),
    I2d(1, 1),
    I2d(-1, 1),
    I2d(1, -1),
];

pub const fn symmetric_grid<const SIZE: usize>() -> [[bool; SIZE]; SIZE] {
    grid::<SIZE, SIZE>()
}
pub const fn grid<const SIZE_X: usize, const SIZE_Y: usize>() -> [[bool; SIZE_X]; SIZE_Y] {
    [[false; SIZE_X]; SIZE_Y]
}
