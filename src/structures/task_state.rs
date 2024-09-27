//! Definitions and interfaces to interact with `x86` and `x86_64` task state segments.

/// A 64-bit task state segment.
///
/// This contains stacks to which to switch when entering a [`PrivilegeLevel`], stacks to which to
/// switch upon an interrupt, and a I/O permsission bitmap.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct TaskStateSegment64 {
    #[doc(hidden)]
    _reserved_0: u32,

    /// Lower 32 bits of the stack pointer for [`PrivilegeLevel::Ring0`].
    rsp0_low: u32,
    /// Upper 32 bits of the stack pointer for [`PrivilegeLevel::Ring0`].
    rsp0_high: u32,

    /// Lower 32 bits of the stack pointer for [`PrivilegeLevel::Ring1`].
    rsp1_low: u32,
    /// Upper 32 bits of the stack pointer for [`PrivilegeLevel::Ring1`].
    rsp1_high: u32,

    /// Lower 32 bits of the stack pointer for [`PrivilegeLevel::Ring2`].
    rsp2_low: u32,
    /// Upper 32 bits of the stack pointer for [`PrivilegeLevel::Ring2`].
    rsp2_high: u32,

    #[doc(hidden)]
    _reserved_1: [u32; 2],

    /// Lower 32 bits of interrupt stack 1.
    ist1_low: u32,
    /// Upper 32 bits of interrupt stack 1.
    ist1_high: u32,

    /// Lower 32 bits of interrupt stack 2.
    ist2_low: u32,
    /// Upper 32 bits of interrupt stack 2.
    ist2_high: u32,

    /// Lower 32 bits of interrupt stack 3.
    ist3_low: u32,
    /// Upper 32 bits of interrupt stack 3.
    ist3_high: u32,

    /// Lower 32 bits of interrupt stack 4.
    ist4_low: u32,
    /// Upper 32 bits of interrupt stack 4.
    ist4_high: u32,

    /// Lower 32 bits of interrupt stack 5.
    ist5_low: u32,
    /// Upper 32 bits of interrupt stack 5.
    ist5_high: u32,

    /// Lower 32 bits of interrupt stack 6.
    ist6_low: u32,
    /// Upper 32 bits of interrupt stack 6.
    ist6_high: u32,

    /// Lower 32 bits of interrupt stack 7.
    ist7_low: u32,
    /// Upper 32 bits of interrupt stack 7.
    ist7_high: u32,

    #[doc(hidden)]
    _reserved_2: [u32; 2],
    #[doc(hidden)]
    _reserved_3: u16,

    /// Offset to the I/O permission bit map from the base of this [`TaskStateSegment64`].
    io_map_base: u16,
}

impl TaskStateSegment64 {
    /// Creates an empty [`TaskStateSegment64`].
    pub const fn new() -> Self {
        Self {
            _reserved_0: 0,
            rsp0_low: 0,
            rsp0_high: 0,
            rsp1_low: 0,
            rsp1_high: 0,
            rsp2_low: 0,
            rsp2_high: 0,
            _reserved_1: [0; 2],
            ist1_low: 0,
            ist1_high: 0,
            ist2_low: 0,
            ist2_high: 0,
            ist3_low: 0,
            ist3_high: 0,
            ist4_low: 0,
            ist4_high: 0,
            ist5_low: 0,
            ist5_high: 0,
            ist6_low: 0,
            ist6_high: 0,
            ist7_low: 0,
            ist7_high: 0,
            _reserved_2: [0; 2],
            _reserved_3: 0,
            io_map_base: 0,
        }
    }

    /// Returns the address of the stack pointer when entering [`PrivilegeLevel::Ring0`].
    pub const fn rsp0(&self) -> u64 {
        (self.rsp0_low as u64) | ((self.rsp0_high as u64) << 32)
    }

    /// Sets the address of the stack pointer when entering [`PrivilegeLevel::Ring0`] to `val`.
    pub const fn set_rsp0(&mut self, val: u64) {
        self.rsp0_low = val as u32;
        self.rsp0_high = (val >> 32) as u32;
    }

    /// Returns the address of the stack pointer when entering [`PrivilegeLevel::Ring1`].
    pub const fn rsp1(&self) -> u64 {
        (self.rsp1_low as u64) | ((self.rsp1_high as u64) << 32)
    }

    /// Sets the address of the stack pointer when entering [`PrivilegeLevel::Ring1`] to `val`.
    pub const fn set_rsp1(&mut self, val: u64) {
        self.rsp1_low = val as u32;
        self.rsp1_high = (val >> 32) as u32;
    }

    /// Returns the address of the stack pointer when entering [`PrivilegeLevel::Ring2`].
    pub const fn rsp2(&self) -> u64 {
        (self.rsp2_low as u64) | ((self.rsp2_high as u64) << 32)
    }

    /// Sets the address of the stack pointer when entering [`PrivilegeLevel::Ring2`] to `val`.
    pub const fn set_rsp2(&mut self, val: u64) {
        self.rsp2_low = val as u32;
        self.rsp2_high = (val >> 32) as u32;
    }

    /// Returns the address of the stack pointer when entering an interrupt handler that uses
    /// interrupt stack 1.
    pub const fn is1(&self) -> u64 {
        (self.ist1_low as u64) | ((self.ist1_high as u64) << 32)
    }

    /// Sets the address of the stack pointer when entering an interrupt handler that uses
    /// interrupt stack 1.
    pub const fn set_ist1(&mut self, val: u64) {
        self.ist1_low = val as u32;
        self.ist1_high = (val >> 32) as u32;
    }

    /// Returns the address of the stack pointer when entering an interrupt handler that uses
    /// interrupt stack 2.
    pub const fn is2(&self) -> u64 {
        (self.ist2_low as u64) | ((self.ist2_high as u64) << 32)
    }

    /// Sets the address of the stack pointer when entering an interrupt handler that uses
    /// interrupt stack 2.
    pub const fn set_ist2(&mut self, val: u64) {
        self.ist2_low = val as u32;
        self.ist2_high = (val >> 32) as u32;
    }

    /// Returns the address of the stack pointer when entering an interrupt handler that uses
    /// interrupt stack 3.
    pub const fn is3(&self) -> u64 {
        (self.ist3_low as u64) | ((self.ist3_high as u64) << 32)
    }

    /// Sets the address of the stack pointer when entering an interrupt handler that uses
    /// interrupt stack 3.
    pub const fn set_ist3(&mut self, val: u64) {
        self.ist3_low = val as u32;
        self.ist3_high = (val >> 32) as u32;
    }

    /// Returns the address of the stack pointer when entering an interrupt handler that uses
    /// interrupt stack 4.
    pub const fn is4(&self) -> u64 {
        (self.ist4_low as u64) | ((self.ist4_high as u64) << 32)
    }

    /// Sets the address of the stack pointer when entering an interrupt handler that uses
    /// interrupt stack 4.
    pub const fn set_ist4(&mut self, val: u64) {
        self.ist4_low = val as u32;
        self.ist4_high = (val >> 32) as u32;
    }

    /// Returns the address of the stack pointer when entering an interrupt handler that uses
    /// interrupt stack 5.
    pub const fn is5(&self) -> u64 {
        (self.ist5_low as u64) | ((self.ist5_high as u64) << 32)
    }

    /// Sets the address of the stack pointer when entering an interrupt handler that uses
    /// interrupt stack 5.
    pub const fn set_ist5(&mut self, val: u64) {
        self.ist5_low = val as u32;
        self.ist5_high = (val >> 32) as u32;
    }

    /// Returns the address of the stack pointer when entering an interrupt handler that uses
    /// interrupt stack 6.
    pub const fn is6(&self) -> u64 {
        (self.ist6_low as u64) | ((self.ist6_high as u64) << 32)
    }

    /// Sets the address of the stack pointer when entering an interrupt handler that uses
    /// interrupt stack 6.
    pub const fn set_ist6(&mut self, val: u64) {
        self.ist6_low = val as u32;
        self.ist6_high = (val >> 32) as u32;
    }

    /// Returns the address of the stack pointer when entering an interrupt handler that uses
    /// interrupt stack 7.
    pub const fn is7(&self) -> u64 {
        (self.ist7_low as u64) | ((self.ist7_high as u64) << 32)
    }

    /// Sets the address of the stack pointer when entering an interrupt handler that uses
    /// interrupt stack 7.
    pub const fn set_ist7(&mut self, val: u64) {
        self.ist7_low = val as u32;
        self.ist7_high = (val >> 32) as u32;
    }

    /// Returns the offset from the base of this [`TaskStateSegment64`] to the start of the I/O
    /// permission bit map.
    pub const fn io_map_base(&self) -> u16 {
        self.io_map_base
    }

    /// Sets the offset from the base of this [`TaskStateSegment64`] to the start of the I/O
    /// permission bit map.
    pub const fn set_io_map_base(&mut self, val: u16) {
        self.io_map_base = val;
    }
}
