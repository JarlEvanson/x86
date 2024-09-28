//! Definitions and interfaces to interact with `x86` and `x86_64`'s processor hardware RNGs.

use core::arch::asm;

use crate::instructions::cpuid::{cpuid, has_cpuid};

/// Interface to the `rdrand` instruction, which is a hardware RNG.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RdRand(());

impl RdRand {
    /// Creates a new [`RdRand`].
    ///
    /// If the `rdrand` instruction is not supported, then this function returns [`None`].
    pub fn new() -> Option<Self> {
        if !has_cpuid() {
            return None;
        }

        // SAFETY:
        // The `cpuid` instruction is supported.
        let cpuid_ebx = unsafe { cpuid(1, 0).ecx };

        #[allow(clippy::nonminimal_bool)]
        if !(cpuid_ebx & (1 << 30) == (1 << 30)) {
            return None;
        }

        Some(Self(()))
    }

    /// Creates a new [`RdRand`] without checking if the `rdrand` instruction is supported.
    ///
    /// # Safety
    /// This processor must support the `rdrand` instruction.
    pub unsafe fn new_unchecked() -> Self {
        debug_assert!(
            Self::new().is_some(),
            "The `rdrand` instruction is not supported"
        );

        Self(())
    }

    /// Returns a random 16-bit number from `rdrand`.
    pub fn get_u16(self) -> Option<u16> {
        let mut rand: u16;
        let success: u8;

        // SAFETY:
        // The `rdrand` instruction is supported.
        unsafe {
            asm!(
                "rdrand {:x}",
                "setc {}",
                lateout(reg) rand,
                lateout(reg_byte) success,
                options(nomem, nostack),
            )
        }

        if success == 0 {
            return None;
        }

        Some(rand)
    }

    /// Returns a random 32-bit number from `rdrand`.
    pub fn get_u32(self) -> Option<u32> {
        let mut rand: u32;
        let success: u8;

        // SAFETY:
        // The `rdrand` instruction is supported.
        unsafe {
            asm!(
                "rdrand {:e}",
                "setc {}",
                lateout(reg) rand,
                lateout(reg_byte) success,
                options(nomem, nostack),
            )
        }

        if success == 0 {
            return None;
        }

        Some(rand)
    }

    /// Returns a random 64-bit number from `rdrand`.
    #[cfg(target_arch = "x86_64")]
    pub fn get_u64(self) -> Option<u64> {
        let mut rand: u64;
        let success: u8;

        // SAFETY:
        // The `rdrand` instruction is supported.
        unsafe {
            asm!(
                "rdrand {}",
                "setc {}",
                lateout(reg) rand,
                lateout(reg_byte) success,
                options(nomem, nostack),
            )
        }

        if success == 0 {
            return None;
        }

        Some(rand)
    }
}

/// Interface to the `rdseed` instruction, which is a hardware RNG.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RdSeed(());

impl RdSeed {
    /// Creates a new [`RdSeed`].
    ///
    /// If the `rdseed` instruction is not supported, then this function returns [`None`].
    pub fn new() -> Option<Self> {
        if !has_cpuid() {
            return None;
        }

        // SAFETY:
        // CPUID is supported.
        let cpuid_ebx = unsafe { cpuid(7, 0).ebx };

        #[allow(clippy::nonminimal_bool)]
        if !(cpuid_ebx & (1 << 18) == (1 << 18)) {
            return None;
        }

        Some(Self(()))
    }

    /// Creates a new [`RdRand`] without checking if the `rdseed` instruction is supported.
    ///
    /// # Safety
    /// This processor must support the `rdseed` instruction.
    pub unsafe fn new_unchecked() -> Self {
        debug_assert!(
            Self::new().is_some(),
            "The `rdseed` instruction is not supported"
        );

        Self(())
    }

    /// Returns a random 16-bit number from `rdseed`.
    pub fn get_u16(self) -> Option<u16> {
        let mut rand: u16;
        let success: u8;

        // SAFETY:
        // The `rdseed` instruction is supported.
        unsafe {
            asm!(
                "rdseed {:x}",
                "setc {}",
                lateout(reg) rand,
                lateout(reg_byte) success,
                options(nomem, nostack),
            )
        }

        if success == 0 {
            return None;
        }

        Some(rand)
    }

    /// Returns a random 32-bit number from `rdseed`.
    pub fn get_u32(self) -> Option<u32> {
        let mut rand: u32;
        let success: u8;

        // SAFETY:
        // The `rdseed` instruction is supported.
        unsafe {
            asm!(
                "rdseed {:e}",
                "setc {}",
                lateout(reg) rand,
                lateout(reg_byte) success,
                options(nomem, nostack),
            )
        }

        if success == 0 {
            return None;
        }

        Some(rand)
    }

    /// Returns a random 64-bit number from `rdseed`.
    #[cfg(target_arch = "x86_64")]
    pub fn get_u64(self) -> Option<u64> {
        let mut rand: u64;
        let success: u8;

        // SAFETY:
        // The `rdseed` instruction is supported.
        unsafe {
            asm!(
                "rdseed {}",
                "setc {}",
                lateout(reg) rand,
                lateout(reg_byte) success,
                options(nomem, nostack),
            )
        }

        if success == 0 {
            return None;
        }

        Some(rand)
    }
}
