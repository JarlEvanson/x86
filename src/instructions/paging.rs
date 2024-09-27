//! Definitions and interfaces for `x86` and `x86_64` instructions related to paging.

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

/// Invalidates the TLB entries for the page of `address` with PCID `pcid`.
///
/// # Safety
/// - The processor must have the `invpcid` CPUID feature.
/// - `pcid` must be a valid PCID.
/// - `address` must be a canonical address.
pub unsafe fn invalid_pcid_address(pcid: u16, address: usize) {
    debug_assert!(pcid < (1 << 12));

    let descriptor = pcid as u128 | ((address as u128) << 64);

    // SAFETY:
    // - 0 is a valid `invalidation_type`.
    // - `picd | (address << 64)` is a valid descriptor value for `invalidation_type` 0.
    unsafe { invpcid(0, &descriptor) }
}

/// Invalidates the TLB entries associated with PCID `pcid`.
///
/// # Safety
/// - The processor must have the `invpcid` CPUID feature.
/// - `pcid` must be a valid PCID.
pub unsafe fn invalidate_pcid(pcid: u16) {
    debug_assert!(pcid < (1 << 12));

    let descriptor = pcid as u128;
    // SAFETY:
    // - 1 is a valid `invalidation_type`.
    // - `pcid` is a valid descriptor value for `invalidation_type` 1.
    unsafe { invpcid(1, &descriptor) }
}

/// Invalidates all TLB entries, including entries marked as global.
///
/// # Safety
/// - The processor must have the `invpcid` CPUID feature.
pub unsafe fn invalidate_non_global() {
    // SAFETY:
    // - 2 is a valid `invalidation_type`.
    // - 0 is a valid descriptor value for `invalidation_type` 2.
    unsafe { invpcid(2, &0) }
}

/// Invalidates all TLB entries, except entries marked as global.
///
/// # Safety
/// - The processor must have the `invpcid` CPUID feature.
pub unsafe fn invalidate() {
    // SAFETY:
    // - 3 is a valid `invalidation_type`.
    // - 0 is a valid descriptor value for `invalidation_type` 3.
    unsafe { invpcid(3, &0) }
}

/// Executes `invpcid` with the given `invalidation_type` using `descriptor`.
///
/// # Safety
/// - `invalidation_type` must be be a valid `invpcid` type.
/// - `descriptor` must be suitable for the `invpcid` call.
unsafe fn invpcid(invalidation_type: usize, descriptor: *const u128) {
    debug_assert!(invalidation_type < 4);
    debug_assert!(crate::instructions::cpuid::has_cpuid());
    // SAFETY:
    // The `cpuid` instruction is available on this processor.
    unsafe { debug_assert!((crate::instructions::cpuid::cpuid(0x7, 0x0).ebx >> 10) & 0b1 == 1) }

    // SAFETY:
    // The processor supports the `invpcid` CPUID feature.
    unsafe {
        core::arch::asm!(
            "invpcid {}, [{}]",
            in(reg) invalidation_type,
            in(reg) descriptor,
            options(readonly, nostack, preserves_flags)
        )
    }
}
