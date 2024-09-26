//! Definitions and interfaces to interact with `x86` and `x86_64`'s CPUID instruction.

use core::arch::asm;

/// Returns `true` if the processor supports the `cpuid` instruction.
pub fn has_cpuid() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        true
    }
    #[cfg(target_arch = "x86")]
    {
        let result: u32;

        // SAFETY:
        // The assembly commands obey expectations and only use compiler allocated registers.
        unsafe {
            asm!(
                "pushfd",
                "pop {flags:e}",
                "mov {save:e}, {flags:e}",
                "xor {flags:e}, 0x200000",
                "push {flags:e}",
                "popfd",
                "pushfd",
                "pop {flags:e}",
                "xor {flags:e}, {save}",
                flags = lateout(reg) result,
                save = lateout(reg) _,
            )
        }

        (result & 0x20_0000) == 0x20_0000
    }
}

/// Returns the [`Cpuid`] associated with `leaf` and `subleaf` on this processor.
///
/// # Safety
/// The `cpuid` instruction must be safe to perform on this processor.
pub unsafe fn cpuid(leaf: u32, subleaf: u32) -> Cpuid {
    debug_assert!(has_cpuid());

    let eax;
    let ebx;
    let ecx;
    let edx;

    // SAFETY:
    // The `cpuid` instruction is safe to perform on this processor.
    unsafe {
        #[cfg(target_arch = "x86")]
        asm!(
            "mov {scratch}, ebx",
            "cpuid",
            "xchg {scratch}, ebx",
            scratch = out(reg) ebx,
            inout("eax") leaf => eax,
            inout("ecx") subleaf => ecx,
            lateout("edx") edx,
            options(nostack, preserves_flags)
        );
        #[cfg(target_arch = "x86_64")]
        asm!(
            "mov {scratch:r}, rbx",
            "cpuid",
            "xchg {scratch:r}, rbx",
            scratch = out(reg) ebx,
            inout("eax") leaf => eax,
            inout("ecx") subleaf => ecx,
            lateout("edx") edx,
            options(nostack, preserves_flags)
        );
    }

    Cpuid { eax, ebx, ecx, edx }
}

/// Result of performing a `cpuid` instruction.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cpuid {
    /// Value stored into eax.
    pub eax: u32,
    /// Value stored into ebx.
    pub ebx: u32,
    /// Value stored into ecx.
    pub ecx: u32,
    /// Value stored into edx.
    pub edx: u32,
}
