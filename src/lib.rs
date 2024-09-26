//! Definitions and interfaces to interact with `x86` and `x86_64` specific instructions,
//! registers, and structures.

#![cfg_attr(not(test), no_std)]

use core::fmt;

pub mod registers;

/// A protection ring level.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrivilegeLevel {
    /// The most privileged ring.
    ///
    /// This is where critical system-software that requires direct hardware access runs. These
    /// programs include the BIOS and interrupt handlers.
    Ring0 = 0,
    /// A moderatley high privileged ring.
    ///
    /// The actual privileges of this level are defined by the supervisor level code.
    Ring1 = 1,
    /// A moderately low privileged ring.
    ///
    /// The actual privileges of this level are defined by the supervisor level code.
    Ring2 = 2,
    /// The least privileged ring.
    ///
    /// This is where application software runs. Access to hardware resources is normally
    /// abstracted at this level, and applications request access to the hardware by calling higher
    /// [`PrivilegeLevel`] code.
    Ring3 = 3,
}

impl PrivilegeLevel {
    /// Creates a [`PrivilegeLevel`] from a numeric value.
    ///
    /// This function returns [`None`] if `val` is greater than 3.
    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::Ring0),
            1 => Some(Self::Ring1),
            2 => Some(Self::Ring2),
            3 => Some(Self::Ring3),
            _ => None,
        }
    }
}

impl fmt::Display for PrivilegeLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
