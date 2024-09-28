//! Definitions and interfaces to interact with `x86` and `x86_64`'s I/O ports.

use core::arch::asm;

/// Writes `val` to `port`.
///
/// # Safety
/// Writing to `port` must not cause undefined behavior.
pub unsafe fn out_u8(port: u16, val: u8) {
    // SAFETY:
    // According to the invariants of the function, this is safe to run.
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") val,
            options(nomem, preserves_flags)
        )
    }
}

/// Writes `val` to `port`.
///
/// # Safety
/// Writing to `port` must not cause undefined behavior.
pub unsafe fn out_u16(port: u16, val: u16) {
    // SAFETY:
    // According to the invariants of the function, this is safe to run.
    unsafe {
        asm!(
            "out dx, ax",
            in("dx") port,
            in("ax") val,
            options(nomem, preserves_flags)
        )
    }
}

/// Writes `val` to `port`.
///
/// # Safety
/// Writing to `port` must not cause undefined behavior.
pub unsafe fn out_u32(port: u16, val: u32) {
    // SAFETY:
    // According to the invariants of the function, this is safe to run.
    unsafe {
        asm!(
            "out dx, eax",
            in("dx") port,
            in("eax") val,
            options(nomem, preserves_flags)
        )
    }
}

/// Reads a `u8` from `port`.
///
/// # Safety
/// Reading from `port` must not cause undefined behavior.
pub unsafe fn in_u8(port: u16) -> u8 {
    let val: u8;
    // SAFETY:
    // According to the invariants of this function, this is safe to run.
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            lateout("al") val,
            options(nomem, preserves_flags)
        )
    }

    val
}

/// Reads a `u16` from `port`.
///
/// # Safety
/// Reading from `port` must not cause undefined behavior.
pub unsafe fn in_u16(port: u16) -> u16 {
    let val: u16;
    // SAFETY:
    // According to the invariants of this function, this is safe to run.
    unsafe {
        asm!(
            "in ax, dx",
            in("dx") port,
            lateout("ax") val,
            options(nomem, preserves_flags)
        )
    }

    val
}

/// Reads a `u32` from `port`.
///
/// # Safety
/// Reading from `port` must not cause undefined behavior.
pub unsafe fn in_u32(port: u16) -> u32 {
    let val: u32;
    // SAFETY:
    // According to the invariants of this function, this is safe to run.
    unsafe {
        asm!(
            "in ax, dx",
            in("dx") port,
            lateout("eax") val,
            options(nomem, preserves_flags)
        )
    }

    val
}
