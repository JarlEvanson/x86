//! Definitions and interfaces to interact with `x86` and `x86_64` specific instructions,
//! registers, and structures.

#![cfg_attr(not(test), no_std)]

// Ensure that at least one of the architecture is enabled.
#[cfg(not(any(feature = "x86", feature = "x86_64")))]
compile_error!("At least one of `x86` and `x86_64` must be enabled");

// Ensure that if feature `instructions` is enabled, the target architecture is one of the
// supported architectures.
#[cfg(all(
    feature = "instructions",
    not(any(target_arch = "x86", target_arch = "x86_64",))
))]
compile_error!("Feature `instructions` mandates compilation for a supported architecture");

// Ensure that if feature `instructions` is enabled, the target architecture's feature is enabled.
#[cfg(all(
    feature = "instructions",
    not(any(
        all(target_arch = "x86", feature = "x86"),
        all(target_arch = "x86_64", feature = "x86_64")
    ))
))]
compile_error!("Feature `instructions` mandates target architecture's feature is enabled");
