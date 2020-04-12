use std::cmp::Ordering;
use std::convert::From;
use std::fmt;
use std::ops::{Add, Sub, AddAssign, SubAssign, Deref};

#[derive(Clone, Copy, Debug, Default, Eq, Ord)]
pub struct Address(u16);

impl Address {
    pub fn new(address: u16) -> Address {
        Address(address)
    }

    pub fn zero() -> Address {
        Address::new(0)
    }

    pub fn max_value() -> Address {
        Address::new(u16::max_value())
    }
}

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Address {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add<u16> for Address {
    type Output = Self;

    fn add(self, other: u16) -> Self::Output {
        Address(self.0 + other)
    }
}

impl Sub<u16> for Address {
    type Output = Self;

    fn sub(self, other: u16) -> Self::Output {
        Address(self.0 - other)
    }
}

impl AddAssign<u16> for Address {
    fn add_assign(&mut self, other: u16) {
        self.0 += other
    }
}

impl SubAssign<u16> for Address {
    fn sub_assign(&mut self, other: u16) {
        self.0 -= other
    }
}

impl From<[u8; 2]> for Address {
    fn from(bytes: [u8; 2]) -> Address {
        Address(u16::from_le_bytes(bytes))
    }
}

impl From<Address> for [u8; 2] {
    fn from(address: Address) -> [u8; 2] {
        address.0.to_le_bytes()
    }
}

impl From<Address> for usize {
    fn from(address: Address) -> usize {
        address.0 as usize
    }
}

impl Deref for Address {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

