//! Definitions and interfaces for interacting with `x86` and `x86_64` descriptors.

use core::marker::PhantomData;

use crate::{Architecture, PrivilegeLevel, X86, X86_64};

/// A non-system descriptor.
#[repr(transparent)]
pub struct Descriptor<A, T, S = Present> {
    /// The value of the descriptor.
    value: u64,
    /// Phantom data to ensure type safety.
    descriptor_type: PhantomData<(A, T, S)>,
}

impl<A: ArchitectureDescriptorExt, T: DescriptorType, S: DescriptorState> Descriptor<A, T, S> {
    /// Returns `true` if the [`Descriptor`]'s present bit is set.
    pub const fn is_present(self) -> bool {
        self.value & (1 << 47) == (1 << 47)
    }

    /// Returns this [`Descriptor`], but with its present bit cleared.
    pub const fn set_present(self) -> Descriptor<A, T, Present> {
        Descriptor {
            value: (self.value & !(1 << 47)) | (1 << 47),
            descriptor_type: PhantomData,
        }
    }

    /// Returns this [`Descriptor`], but with its present bit set.
    pub const fn clear_present(self) -> Descriptor<A, T, NotPresent> {
        Descriptor {
            value: self.value & !(1 << 47),
            descriptor_type: PhantomData,
        }
    }

    /// Returns the [`PrivilegeLevel`] of this [`Descriptor`].
    pub const fn dpl(self) -> PrivilegeLevel {
        match PrivilegeLevel::from_u8((self.value >> 45) as u8 & 0b11) {
            Some(dpl) => dpl,
            None => unreachable!(),
        }
    }

    /// Returns this [`Descriptor`], but with its [`PrivilegeLevel`] set to `dpl`.
    pub const fn set_dpl(mut self, dpl: PrivilegeLevel) -> Self {
        self.value = (self.value & !(0b11 << 45)) | ((dpl as u64) << 45);
        self
    }

    /// Returns the raw representation of this [`Descriptor`].
    pub const fn to_raw(self) -> u64 {
        self.value
    }
}

impl<A: ArchitectureDescriptorExt, T: DescriptorType> Descriptor<A, T, Present> {
    /// Returns whether the limit field of this [`Descriptor`] should be scaled by 4096.
    pub const fn granularity(self) -> bool {
        self.value & (1 << 55) == (1 << 55)
    }

    /// Returns this [`Descriptor`], but with its granularity setting set to `granularity`.
    pub const fn set_granularity(mut self, granularity: bool) -> Self {
        self.value = (self.value & !(1 << 55)) | ((granularity as u64) << 55);
        self
    }

    /// Returns the base address of this [`Descriptor`].
    pub const fn address(self) -> u32 {
        (((self.value >> 16) & 0xFF_FFFF) | (self.value >> 32)) as u32
    }

    /// Returns this [`Descriptor`], but with its base address set to `address`.
    pub const fn set_address(mut self, address: u32) -> Self {
        self.value = (self.value & !((0xFF << 56) | (0xFF_FFFF << 16)))
            | ((address as u64 & 0xFF_FFFF) << 16)
            | ((address as u64 & (0xFF << 24)) << 32);
        self
    }

    /// Returns the base address of this [`Descriptor`].
    pub const fn limit(self) -> u32 {
        (self.value & 0xFFFF | ((self.value >> 48) & 0xF)) as u32
    }

    /// Returns this [`Descriptor`], but with its limit set to `limit`.
    pub const fn set_limit(mut self, limit: u32) -> Option<Self> {
        #[allow(clippy::nonminimal_bool)]
        if !(limit < (1 << 20)) {
            return None;
        }

        self.value = (self.value & !((0xF << 48) | 0xFFFF))
            | (limit as u64 & 0xFFFF)
            | ((limit as u64 & (0xF << 16)) << 48);
        Some(self)
    }
}

impl<A, T, S> Clone for Descriptor<A, T, S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<A, T, S> Copy for Descriptor<A, T, S> {}

/// A code segment.
pub struct Data;
/// A data segment.
pub struct Code;

/// The type of a [`Descriptor`].
pub trait DescriptorType: private::DescriptorTypeSealed {}
impl DescriptorType for Data {}
impl DescriptorType for Code {}

/// A [`Descriptor`] that has not been classified as [`Present`] or [`NotPresent`].
pub struct Unclassified;
/// A [`Descriptor`] that is not present.
pub struct NotPresent;
/// A [`Descriptor`] that is present.
pub struct Present;

/// The state of a [`Descriptor`].
pub trait DescriptorState: private::DescriptorStateSealed {}
impl DescriptorState for NotPresent {}
impl DescriptorState for Present {}

/// Extension trait for [`Descriptor`] interaction.
pub trait ArchitectureDescriptorExt: Architecture {}
impl ArchitectureDescriptorExt for X86 {}
impl ArchitectureDescriptorExt for X86_64 {}

mod private {
    //! Module used to seal various traits.

    use super::{Code, Data, NotPresent, Present};

    /// Trait sealing [`DescriptorType`].
    pub trait DescriptorTypeSealed {}
    impl DescriptorTypeSealed for Code {}
    impl DescriptorTypeSealed for Data {}

    /// Trait sealing [`DescriptorState`].
    pub trait DescriptorStateSealed {}
    impl DescriptorStateSealed for NotPresent {}
    impl DescriptorStateSealed for Present {}
}
