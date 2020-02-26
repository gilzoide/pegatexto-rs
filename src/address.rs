use std::cmp::Ordering;
use std::convert::From;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Eq, Ord)]
pub struct Address(u16);

impl Address {
    pub fn new(address: u16) -> Address {
        Address(address)
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

