//! Definitions and interfaces to interact with `x86` and `x86_64` specific instructions,
//! registers, and structures.

#![cfg_attr(not(test), no_std)]

use core::fmt;

#[cfg(feature = "instructions")]
pub mod instructions;
pub mod registers;
pub mod structures;

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

/// The `x86` architecture.
///
/// This is a 32-bit architecture.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct X86;

/// The `x86_64` architecture.
///
/// This is a 64-bit architecture.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct X86_64;

/// The supported architectures.
///
/// This trait is sealed.
pub trait Architecture: private::Sealed {
    /// The type of a general purpose register.
    type GeneralRegister: Copy
        + core::ops::BitAnd<Output = Self::GeneralRegister>
        + core::ops::BitOr<Output = Self::GeneralRegister>
        + core::ops::BitXor<Output = Self::GeneralRegister>
        + core::hash::Hash
        + PartialEq
        + Eq
        + PartialOrd
        + Ord;
}

impl Architecture for X86 {
    type GeneralRegister = u32;
}
impl Architecture for X86_64 {
    type GeneralRegister = u64;
}

mod private {
    //! Module used to seal the [`Architecture`] trait.

    use crate::{X86, X86_64};

    /// Trait used to seal [`Architecture`].
    pub trait Sealed {}

    // Implement for the two architectures supported.
    impl Sealed for X86 {}
    impl Sealed for X86_64 {}
}
