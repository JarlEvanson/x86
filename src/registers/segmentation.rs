//! Definitions and interfaces to interact with `x86_64` segment registers.

use core::fmt;

#[cfg(feature = "instructions")]
use core::arch::asm;

use crate::PrivilegeLevel;

/// The code segment register.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CS;

impl CS {
    /// Returns the current value of the [`CS`] register.
    #[cfg(feature = "instructions")]
    pub fn get() -> SegmentSelector {
        let segment_selector: u16;
        // SAFETY:
        // Reading from the code segment register does not adversely affect the processor.
        unsafe {
            asm!(
                "mov {:x}, cs",
                out(reg) segment_selector,
                options(nomem, nostack, preserves_flags)
            )
        }

        SegmentSelector(segment_selector)
    }

    /// Loads the [`CS`] register with the given [`SegmentSelector`].
    ///
    /// # Safety
    /// - `selector` refers to a valid [`SegmentDescriptor`][sd].
    /// - Loading [`CS`] with `selector` will not cause undefined behavior.
    ///
    /// [sd]: crate::structures::segment_table::SegmentDescriptor
    #[cfg(feature = "instructions")]
    pub unsafe fn set(selector: SegmentSelector) {
        // SAFETY:
        // The assembly code does well-formed operations and follows its declared options.
        #[cfg(target_arch = "x86")]
        unsafe {
            asm!(
                "push {selector:x}",
                "push 5f",
                "retf",
                "5:",
                selector = in(reg) selector.0
            )
        }
        // SAFETY:
        // The assembly code does well-formed operations and follows its declared options.
        #[cfg(target_arch = "x86_64")]
        unsafe {
            asm!(
                "push {selector}",
                "lea {ret_address}, [rip + 5f]",
                "push {ret_address}",
                "retfq",
                "5:",
                selector = in(reg) u64::from(selector.0),
                ret_address = lateout(reg) _,
                options(preserves_flags)
            )
        }
    }
}

/// The data segment register.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DS;

impl DS {
    /// Returns the current value of the [`DS`] register.
    #[cfg(feature = "instructions")]
    pub fn get() -> SegmentSelector {
        let segment_selector: u16;
        // SAFETY:
        // Reading from the data segment register does not adversely affect the processor.
        unsafe {
            asm!(
                "mov {:x}, ds",
                out(reg) segment_selector,
                options(nomem, nostack, preserves_flags)
            )
        }

        SegmentSelector(segment_selector)
    }

    /// Loads the [`DS`] register with the given [`SegmentSelector`].
    ///
    /// # Safety
    /// - `selector` refers to a valid [`SegmentDescriptor`][sd].
    /// - Loading [`DS`] with `selector` will not cause undefined behavior.
    ///
    /// [sd]: crate::structures::segment_table::SegmentDescriptor
    #[cfg(feature = "instructions")]
    pub unsafe fn set(selector: SegmentSelector) {
        // SAFETY:
        // The assembly code does well-formed operations and follows its declared options.
        unsafe {
            asm!(
                "mov ds, {:x}",
                in(reg) selector.0,
                options(nomem, nostack, preserves_flags)
            )
        }
    }
}

/// The stack segment register.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SS;

impl SS {
    /// Returns the current value of the [`SS`] register.
    #[cfg(feature = "instructions")]
    pub fn get() -> SegmentSelector {
        let segment_selector: u16;
        // SAFETY:
        // Reading from the stack segment register does not adversely affect the processor.
        unsafe {
            asm!(
                "mov {:x}, ss",
                out(reg) segment_selector,
                options(nomem, nostack, preserves_flags)
            )
        }

        SegmentSelector(segment_selector)
    }

    /// Loads the [`SS`] register with the given [`SegmentSelector`].
    ///
    /// # Safety
    /// - `selector` refers to a valid [`SegmentDescriptor`][sd].
    /// - Loading [`SS`] with `selector` will not cause undefined behavior.
    ///
    /// [sd]: crate::structures::segment_table::SegmentDescriptor
    #[cfg(feature = "instructions")]
    pub unsafe fn set(selector: SegmentSelector) {
        // SAFETY:
        // The assembly code does well-formed operations and follows its declared options.
        unsafe {
            asm!(
                "mov ss, {:x}",
                in(reg) selector.0,
                options(nomem, nostack, preserves_flags)
            )
        }
    }
}

/// The ES register.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ES;

impl ES {
    /// Returns the current value of the [`ES`] register.
    #[cfg(feature = "instructions")]
    pub fn get() -> SegmentSelector {
        let segment_selector: u16;
        // SAFETY:
        // Reading from the ES register does not adversely affect the processor.
        unsafe {
            asm!(
                "mov {:x}, es",
                out(reg) segment_selector,
                options(nomem, nostack, preserves_flags)
            )
        }

        SegmentSelector(segment_selector)
    }

    /// Loads the [`ES`] register with the given [`SegmentSelector`].
    ///
    /// # Safety
    /// - `selector` refers to a valid [`SegmentDescriptor`][sd].
    /// - Loading [`ES`] with `selector` will not cause undefined behavior.
    ///
    /// [sd]: crate::structures::segment_table::SegmentDescriptor
    #[cfg(feature = "instructions")]
    pub unsafe fn set(selector: SegmentSelector) {
        // SAFETY:
        // The assembly code does well-formed operations and follows its declared options.
        unsafe {
            asm!(
                "mov es, {:x}",
                in(reg) selector.0,
                options(nomem, nostack, preserves_flags)
            )
        }
    }
}

/// The FS register.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FS;

impl FS {
    /// Returns the current value of the [`FS`] register.
    #[cfg(feature = "instructions")]
    pub fn get() -> SegmentSelector {
        let segment_selector: u16;
        // SAFETY:
        // Reading from the FS register does not adversely affect the processor.
        unsafe {
            asm!(
                "mov {:x}, fs",
                out(reg) segment_selector,
                options(nomem, nostack, preserves_flags)
            )
        }

        SegmentSelector(segment_selector)
    }

    /// Loads the [`FS`] register with the given [`SegmentSelector`].
    ///
    /// # Safety
    /// - `selector` refers to a valid [`SegmentDescriptor`][sd].
    /// - Loading [`FS`] with `selector` will not cause undefined behavior.
    ///
    /// [sd]: crate::structures::segment_table::SegmentDescriptor
    #[cfg(feature = "instructions")]
    pub unsafe fn set(selector: SegmentSelector) {
        // SAFETY:
        // The assembly code does well-formed operations and follows its declared options.
        unsafe {
            asm!(
                "mov fs, {:x}",
                in(reg) selector.0,
                options(nomem, nostack, preserves_flags)
            )
        }
    }
}

/// The GS register.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GS;

impl GS {
    /// Returns the current value of the [`GS`] register.
    #[cfg(feature = "instructions")]
    pub fn get() -> SegmentSelector {
        let segment_selector: u16;
        // SAFETY:
        // Reading from the GS register does not adversely affect the processor.
        unsafe {
            asm!(
                "mov {:x}, gs",
                out(reg) segment_selector,
                options(nomem, nostack, preserves_flags)
            )
        }

        SegmentSelector(segment_selector)
    }

    /// Loads the [`GS`] register with the given [`SegmentSelector`].
    ///
    /// # Safety
    /// - `selector` refers to a valid [`SegmentDescriptor`][sd].
    /// - Load [`GS`] with `selector` will not cause undefined behavior.
    ///
    /// [sd]: crate::structures::segment_table::SegmentDescriptor
    #[cfg(feature = "instructions")]
    pub unsafe fn set(selector: SegmentSelector) {
        // SAFETY:
        // The assembly code does well-formed operations and follows its declared options.
        unsafe {
            asm!(
                "mov gs, {:x}",
                in(reg) selector.0,
                options(nomem, nostack, preserves_flags)
            )
        }
    }
}

/// Sepcifies from where to load an element into a segment register.
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SegmentSelector(u16);

impl SegmentSelector {
    /// Creates a new [`SegmentSelector`].
    pub const fn new(index: u16, ldt: bool, rpl: PrivilegeLevel) -> Self {
        assert!(index < 8096);

        Self((index << 3) | ((ldt as u16) << 2) | (rpl as u16))
    }

    /// Returns the index of the [`SegmentSelector`] into the GDT or LDT.
    pub const fn index(self) -> u16 {
        self.0 >> 3
    }

    /// Returns whether this [`SegmentSelector`] refers to the GDT or LDT.
    pub const fn ldt(self) -> bool {
        self.0 & (1 << 2) == (1 << 2)
    }

    /// Returns the requested [`PrivilegeLevel`] of this [`SegmentSelector`].
    pub const fn rpl(self) -> PrivilegeLevel {
        match self.0 & 0b11 {
            0 => PrivilegeLevel::Ring0,
            1 => PrivilegeLevel::Ring1,
            2 => PrivilegeLevel::Ring2,
            3 => PrivilegeLevel::Ring3,
            _ => unreachable!(),
        }
    }

    /// Sets the index of this [`SegmentSelector`] into the GDT or LDT.
    pub const fn set_index(self, index: u16) -> Self {
        assert!(index < 8096);

        Self((index << 3) | (self.0 & 0b111))
    }

    /// Sets whether this [`SegmentSelector`] refers to the GDT or LDT.
    pub const fn set_ldt(self, ldt: bool) -> Self {
        Self((self.0 & !0b100) | ((ldt as u16) << 2))
    }

    /// Sets the requested [`PrivilegeLevel`] of this [`SegmentSelector`].
    pub const fn set_rpl(self, rpl: PrivilegeLevel) -> Self {
        Self((self.0 & !0b11) | (rpl as u16))
    }

    /// Creates a new [`SegmentSelector`] from its raw representation.
    pub const fn from_raw(raw: u16) -> Self {
        Self(raw)
    }

    /// Returns the raw representation of this [`SegmentSelector`].
    pub const fn to_raw(self) -> u16 {
        self.0
    }
}

impl fmt::Debug for SegmentSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("SegmentSelector");

        debug_struct.field("index", &self.index());
        debug_struct.field("ldt", &self.ldt());
        debug_struct.field("rpl", &self.rpl());

        debug_struct.finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::{registers::segmentation::SegmentSelector, PrivilegeLevel};

    #[test]
    fn segment_selector_roundtrip() {
        let index = 3753;
        let ldt = true;
        let rpl = PrivilegeLevel::Ring3;

        let segment_selector = SegmentSelector::new(index, ldt, rpl);

        assert_eq!(segment_selector.index(), index);
        assert_eq!(segment_selector.ldt(), ldt);
        assert_eq!(segment_selector.rpl(), rpl);
    }

    #[test]
    #[should_panic]
    fn segment_out_of_range() {
        SegmentSelector::new(u16::MAX, true, PrivilegeLevel::Ring3);
    }
}
