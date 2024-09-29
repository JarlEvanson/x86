//! Definitions and interfaces to interact with `x86` and `x86_64`'s model specific registers.

/// Reads 64-bits from the specific register located at `address`.
///
/// # Safety
/// The caller must ensure that this read operation does not cause unsafety.
#[cfg(feature = "instructions")]
pub unsafe fn read_msr(address: u32) -> u64 {
    let eax: u32;
    let edx: u32;

    // SAFETY:
    // The invariants of this function cause this assembly block to be safe.
    unsafe {
        core::arch::asm!(
            "rdmsr",
            in("ecx") address,
            out("eax") eax,
            out("edx") edx,
            options(nostack, preserves_flags),
        )
    }

    eax as u64 | ((edx as u64) << 32)
}

/// Writes `value` into the model specific register located at `address`.
///
/// # Safety
/// The caller must ensure that this write operation does not cause unsafety.
#[cfg(feature = "instructions")]
pub unsafe fn write_msr(address: u32, value: u64) {
    // SAFETY:
    // The invariants of this function cause this assembly block to be safe.
    unsafe {
        core::arch::asm!(
            "wrmsr",
            in("ecx") address,
            in("eax") value as u32,
            in("edx") (value >> 32) as u32,
            options(nostack, preserves_flags),
        )
    }
}
