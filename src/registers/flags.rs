//! Definitions and interfaces to interact with the `x86` and `x86_64` flags register.

use core::fmt;

use crate::{Architecture, PrivilegeLevel, X86, X86_64};

/// The `x86` and `x86_64` flags register.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Flags<A: Architecture>(A::GeneralRegister);

#[allow(clippy::missing_docs_in_private_items)]
impl<A: Architecture> Flags<A> {
    const CARRY_BIT: usize = 0;
    const PARITY_BIT: usize = 2;
    const AUXILIARY_CARRY_BIT: usize = 4;
    const ZERO_BIT: usize = 6;
    const SIGN_BIT: usize = 8;
    const OVERFLOW_BIT: usize = 11;

    const TRAP_BIT: usize = 8;
    const INTERRUPT_ENABLE_BIT: usize = 9;
    const DIRECTION_BIT: usize = 10;
    const NESTED_TASK_BIT: usize = 14;
    const RESUME_BIT: usize = 16;
    const VIRTUAL_8086_BIT: usize = 17;
    const ALIGNMENT_CHECK_BIT: usize = 18;

    const VIRTUAL_INTERRUPT_BIT: usize = 19;
    const VIRTUAL_INTERRUPT_PENDING_BIT: usize = 20;
    const IDENTIFICATION_BIT: usize = 21;

    const IOPL_START: usize = 12;
}

impl<A: ArchitectureExt> Flags<A> {
    /// Set by hardware if the last operation generated a carry or borrow out of the
    /// most-significant bit of the result; cleared otherwise.
    ///
    /// This flag indicates an overflow condition for unsigned-integer arithmetic.
    pub const CARRY: Self = Self(A::CARRY_BIT);

    /// Set if the least-significant byte of the result contains an even number of 1 bits; cleared
    /// otherwise.
    pub const PARITY: Self = Self(A::PARITY_BIT);
    /// Set if an operation generated a carry or borrow out of bit 3 of the result; cleared
    /// otherwise.
    ///
    /// This flag is used in binary-coded decimal arithmetic.
    pub const AUXILIARY_CARRY: Self = Self(A::AUXILIARY_CARRY_BIT);
    /// Set if the result is zero; cleared otherwise.
    pub const ZERO: Self = Self(A::ZERO_BIT);
    /// Set equal to the most-significant bit of the result, which is the sign bit of a signed
    /// integer.
    pub const SIGN: Self = Self(A::SIGN_BIT);
    /// Set if the integer result is too large a positive number or too small a negative number to
    /// fit in the destination operand; cleared otherwise.
    ///
    /// This flag indicates an overflow condition for signed-integer arithmetic.
    pub const OVERFLOW: Self = Self(A::OVERFLOW_BIT);

    /// Controls the direction of string operations.
    ///
    /// If set, string operations auto-decrement; otherwise string operations auto-increment.
    pub const DIRECTION: Self = Self(A::DIRECTION_BIT);

    // System Flags

    /// Set to enable single-step mode for debugging; clear to disable single-step mode.
    pub const TRAP: Self = Self(A::TRAP_BIT);

    /// Controls the response of the processor to maskable interrupt requests.
    ///
    /// If set, the processor responds to maskable interrupts; otherwise maskable interrupts are
    /// inhibited.
    pub const INTERRUPT_ENABLE: Self = Self(A::INTERRUPT_ENABLE_BIT);
    /// Controls the chaining of interrupted and called tasks.
    ///
    /// Set when the current task is linked to the previously executed task; cleared when the
    /// current task is not linked to another task.
    pub const NESTED_TASK: Self = Self(A::NESTED_TASK_BIT);
    /// Controls the processor's response to debug exceptions.
    pub const RESUME: Self = Self(A::RESUME_BIT);
    /// Set to enable virtual-8086 mode; clear to return to protected mode without virtual-8086
    /// mode semantics.
    pub const VIRTUAL_8086_MODE: Self = Self(A::VIRTUAL_8086_BIT);
    /// If the AM bit is set in the CR0 register, alignment checking of user-mode data accesses is
    /// enabled if and only if this flag is set.
    ///
    /// If the SMAP bit is set in the CR4 register, explicit supervisor-mode data accesses to
    /// user-mode pages are allowed if and only if this bit is 1.
    pub const ALIGNMENT_CHECK: Self = Self(A::ALIGNMENT_CHECK_BIT);
    /// Virtual image of the [`Flags::INTERRUPT_ENABLE`].
    ///
    /// Used in conjunction with the [`Flags::VIRTUAL_INTERRUPT_PENDING`] flag.
    pub const VIRTUAL_INTERRUPT: Self = Self(A::VIRTUAL_INTERRUPT_BIT);

    /// Set to indicate that an interrupt is pending; clear when no interrupt is pending.
    ///
    /// This is set and cleared by software; the processor only reads it.
    pub const VIRTUAL_INTERRUPT_PENDING: Self = Self(A::VIRTUAL_INTERRUPT_PENDING_BIT);

    /// The ability of a program to set or clear this flag indicates support for the CPUID
    /// instruction.
    pub const IDENTIFICATION: Self = Self(A::IDENTIFICATION_BIT);

    /// Indicates the I/O privilege level of the currently running program.
    ///
    /// The CPL of the currently running program must be less than or equal to the IOPL to access
    /// the IO address space.
    pub const IOPL: Self = Self(A::IOPL_BITS);

    /// Returns the I/O [`PrivilegeLevel`] of the currently running program.
    pub fn iopl(&self) -> PrivilegeLevel {
        A::iopl(self.0)
    }
}

#[cfg(all(feature = "instructions", target_arch = "x86"))]
impl Flags<X86> {
    /// Returns the current value of the [`Flags`] register.
    pub fn get() -> Self {
        let value;

        unsafe {
            core::arch::asm!(
                "pushfd",
                "pop {}",
                lateout(reg) value,
                options(preserves_flags),
            );
        }

        Self(value)
    }

    /// Sets the [`Flags`] register.
    ///
    /// # Safety
    /// - Loading `flags` will not cause undefined behavior.
    pub unsafe fn set(flags: Self) {
        unsafe {
            core::arch::asm!(
                "push {}",
                "popfd",
                inlateout(reg) flags.0 => _,
            )
        }
    }
}

#[cfg(all(feature = "instructions", target_arch = "x86_64"))]
impl Flags<X86_64> {
    /// Returns the current value of the [`Flags`] register.
    pub fn get() -> Self {
        let value;

        // SAFETY:
        unsafe {
            core::arch::asm!(
                "pushfq",
                "pop {}",
                lateout(reg) value,
                options(preserves_flags),
            );
        }

        Self(value)
    }

    /// Sets the [`Flags`] register.
    ///
    /// # Safety
    /// - Loading `flags` will not cause undefined behavior.
    pub unsafe fn set(flags: Self) {
        // SAFETY:
        unsafe {
            core::arch::asm!(
                "push {}",
                "popfq",
                inlateout(reg) flags.0 => _,
            )
        }
    }
}

impl<T: ArchitectureExt> core::ops::BitAnd for Flags<T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl<T: ArchitectureExt> core::ops::BitAndAssign for Flags<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0);
    }
}

impl<T: ArchitectureExt> core::ops::BitOr for Flags<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl<T: ArchitectureExt> core::ops::BitOrAssign for Flags<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0);
    }
}

impl<T: ArchitectureExt> core::ops::BitXor for Flags<T> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl<T: ArchitectureExt> core::ops::BitXorAssign for Flags<T> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0);
    }
}

impl<A: ArchitectureExt + PartialEq + Copy> fmt::Debug for Flags<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        FlagsDisplay::<A>(self.0).fmt(f)
    }
}

impl<A: ArchitectureExt + PartialEq + Copy> fmt::Display for Flags<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// Wrapper to display flags in a cohesive manner.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct FlagsDisplay<A: Architecture>(pub A::GeneralRegister);

impl<A: ArchitectureExt + PartialEq + Copy> fmt::Debug for FlagsDisplay<A> {
    #[allow(unused_assignments)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = Flags::<A>(self.0);
        let mut prev = false;

        /// Generates flag formatting code.
        macro_rules! flag {
            ($condition:expr, $name:expr) => {
                if $condition {
                    if prev {
                        write!(f, " | ")?;
                    }

                    write!(f, "{}", $name)?;
                    prev = true;
                }
            };
        }

        flag!(value & Flags::<A>::CARRY == Flags::<A>::CARRY, "CARRY");
        flag!(value & Flags::<A>::PARITY == Flags::<A>::PARITY, "PARITY");
        flag!(
            value & Flags::<A>::AUXILIARY_CARRY == Flags::<A>::AUXILIARY_CARRY,
            "AUX_CARRY"
        );
        flag!(value & Flags::<A>::ZERO == Flags::<A>::ZERO, "ZERO");
        flag!(value & Flags::<A>::SIGN == Flags::<A>::SIGN, "SIGN");
        flag!(
            value & Flags::<A>::OVERFLOW == Flags::<A>::OVERFLOW,
            "OVERFLOW"
        );

        flag!(
            value & Flags::<A>::DIRECTION == Flags::<A>::DIRECTION,
            "DIRECTION"
        );
        flag!(value & Flags::<A>::TRAP == Flags::<A>::TRAP, "TRAP");
        flag!(
            value & Flags::<A>::INTERRUPT_ENABLE == Flags::<A>::INTERRUPT_ENABLE,
            "INTERRUPT_ENABLE"
        );
        flag!(
            value & Flags::<A>::NESTED_TASK == Flags::<A>::NESTED_TASK,
            "NESTED_TASK"
        );
        flag!(value & Flags::<A>::RESUME == Flags::<A>::RESUME, "RESUME");
        flag!(
            value & Flags::<A>::VIRTUAL_8086_MODE == Flags::<A>::VIRTUAL_8086_MODE,
            "VIRTUAL_8086_MODE"
        );
        flag!(
            value & Flags::<A>::ALIGNMENT_CHECK == Flags::<A>::ALIGNMENT_CHECK,
            "ALIGNMENT_CHECK"
        );
        flag!(
            value & Flags::<A>::VIRTUAL_INTERRUPT == Flags::<A>::VIRTUAL_INTERRUPT,
            "VIRTUAL_INTERRUPT"
        );
        flag!(
            value & Flags::<A>::VIRTUAL_INTERRUPT_PENDING == Flags::<A>::VIRTUAL_INTERRUPT_PENDING,
            "VIRTUAL_INTERRUPT_PENDING"
        );
        flag!(
            value & Flags::<A>::IDENTIFICATION == Flags::<A>::IDENTIFICATION,
            "IDENTIFICATION"
        );

        if prev {
            write!(f, " | ")?;
        }
        write!(f, "IOPL{}", value.iopl() as u8)?;

        Ok(())
    }
}

impl<A: ArchitectureExt + PartialEq + Copy> fmt::Display for FlagsDisplay<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// Macro implementing [`ArchitectureExt`].
macro_rules! impl_arch_flags {
    ($name:ident, $container:ident) => {
        impl ArchitectureExt for $name {
            const CARRY_BIT: Self::GeneralRegister = 1 << Flags::<Self>::CARRY_BIT;
            const PARITY_BIT: Self::GeneralRegister = 1 << Flags::<Self>::PARITY_BIT;
            const AUXILIARY_CARRY_BIT: Self::GeneralRegister =
                1 << Flags::<Self>::AUXILIARY_CARRY_BIT;
            const ZERO_BIT: Self::GeneralRegister = 1 << Flags::<Self>::ZERO_BIT;
            const SIGN_BIT: Self::GeneralRegister = 1 << Flags::<Self>::SIGN_BIT;
            const OVERFLOW_BIT: Self::GeneralRegister = 1 << Flags::<Self>::OVERFLOW_BIT;

            const TRAP_BIT: Self::GeneralRegister = 1 << Flags::<Self>::TRAP_BIT;
            const INTERRUPT_ENABLE_BIT: Self::GeneralRegister =
                1 << Flags::<Self>::INTERRUPT_ENABLE_BIT;
            const DIRECTION_BIT: Self::GeneralRegister = 1 << Flags::<Self>::DIRECTION_BIT;
            const NESTED_TASK_BIT: Self::GeneralRegister = 1 << Flags::<Self>::NESTED_TASK_BIT;
            const RESUME_BIT: Self::GeneralRegister = 1 << Flags::<Self>::RESUME_BIT;
            const VIRTUAL_8086_BIT: Self::GeneralRegister = 1 << Flags::<Self>::VIRTUAL_8086_BIT;
            const ALIGNMENT_CHECK_BIT: Self::GeneralRegister =
                1 << Flags::<Self>::ALIGNMENT_CHECK_BIT;

            const VIRTUAL_INTERRUPT_BIT: Self::GeneralRegister =
                1 << Flags::<Self>::VIRTUAL_INTERRUPT_BIT;
            const VIRTUAL_INTERRUPT_PENDING_BIT: Self::GeneralRegister =
                1 << Flags::<Self>::VIRTUAL_INTERRUPT_PENDING_BIT;
            const IDENTIFICATION_BIT: Self::GeneralRegister =
                1 << Flags::<Self>::IDENTIFICATION_BIT;

            const IOPL_BITS: Self::GeneralRegister = 0b11 << Flags::<Self>::IOPL_START;

            fn iopl(val: Self::GeneralRegister) -> PrivilegeLevel {
                match (val & Self::IOPL_BITS) >> Flags::<Self>::IOPL_START {
                    0 => PrivilegeLevel::Ring0,
                    1 => PrivilegeLevel::Ring1,
                    2 => PrivilegeLevel::Ring2,
                    3 => PrivilegeLevel::Ring3,
                    _ => unreachable!(),
                }
            }
        }
    };
}

/// Extension trait that helps implement [`Flags`].
pub trait ArchitectureExt: Architecture + Sized {
    #[doc(hidden)]
    const CARRY_BIT: Self::GeneralRegister;
    #[doc(hidden)]
    const PARITY_BIT: Self::GeneralRegister;
    #[doc(hidden)]
    const AUXILIARY_CARRY_BIT: Self::GeneralRegister;
    #[doc(hidden)]
    const ZERO_BIT: Self::GeneralRegister;
    #[doc(hidden)]
    const SIGN_BIT: Self::GeneralRegister;
    #[doc(hidden)]
    const OVERFLOW_BIT: Self::GeneralRegister;

    #[doc(hidden)]
    const TRAP_BIT: Self::GeneralRegister;
    #[doc(hidden)]
    const INTERRUPT_ENABLE_BIT: Self::GeneralRegister;
    #[doc(hidden)]
    const DIRECTION_BIT: Self::GeneralRegister;
    #[doc(hidden)]
    const NESTED_TASK_BIT: Self::GeneralRegister;
    #[doc(hidden)]
    const RESUME_BIT: Self::GeneralRegister;
    #[doc(hidden)]
    const VIRTUAL_8086_BIT: Self::GeneralRegister;
    #[doc(hidden)]
    const ALIGNMENT_CHECK_BIT: Self::GeneralRegister;

    #[doc(hidden)]
    const VIRTUAL_INTERRUPT_BIT: Self::GeneralRegister;
    #[doc(hidden)]
    const VIRTUAL_INTERRUPT_PENDING_BIT: Self::GeneralRegister;
    #[doc(hidden)]
    const IDENTIFICATION_BIT: Self::GeneralRegister;

    #[doc(hidden)]
    const IOPL_BITS: Self::GeneralRegister;

    #[doc(hidden)]
    fn iopl(flags: Self::GeneralRegister) -> PrivilegeLevel;
}

#[cfg(feature = "x86")]
impl_arch_flags!(X86, u32);
#[cfg(feature = "x86_64")]
impl_arch_flags!(X86_64, u64);
