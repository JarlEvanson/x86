//! Definitions and interfaces for `x86` and `x86_64` instructions related to managing TLB entries.

/// Invalidates the TLB entries for the page of `address`.
///
/// Executes `invlpg` under the hood.
pub fn invalidate_page(address: usize) {
    // SAFETY:
    // Should not cause any problems if called repeatedly.
    unsafe {
        core::arch::asm!(
            "invlpg [{}]",
            in(reg) address,
            options(nomem, nostack, preserves_flags)
        )
    }
}
