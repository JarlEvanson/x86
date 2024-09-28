//! Definitions and interfaces to interact with `x86` and `x86_64` task state segments.

use crate::registers::segmentation::SegmentSelector;

/// Processor state needed to restore a 16-bit task.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TaskStateSegment16 {
    /// The [`SegmentSelector`] of the [`TaskStateSegment16`] of the previous task.
    pub previous_task_link: SegmentSelector,

    /// THe stack pointer that should be loaded when a transfer occurs to
    /// [`PrivilegeLevel::Ring0`][plr0]
    ///
    /// [plr0]: crate::PrivilegeLevel::Ring0
    pub stack_pointer_0: u16,
    /// The stack [`SegmentSelector`] that should be loaded when a transfer occurs to
    /// [`PrivilegeLevel::Ring0`][plr0].
    ///
    /// [plr0]: crate::PrivilegeLevel::Ring0
    pub stack_segment_0: SegmentSelector,

    /// THe stack pointer that should be loaded when a transfer occurs to
    /// [`PrivilegeLevel::Ring1`][plr1]
    ///
    /// [plr1]: crate::PrivilegeLevel::Ring1
    pub stack_pointer_1: u16,
    /// The stack [`SegmentSelector`] that should be loaded when a transfer occurs to
    /// [`PrivilegeLevel::Ring1`][plr1].
    ///
    /// [plr1]: crate::PrivilegeLevel::Ring1
    pub stack_segment_1: SegmentSelector,

    /// THe stack pointer that should be loaded when a transfer occurs to
    /// [`PrivilegeLevel::Ring2`][plr2]
    ///
    /// [plr2]: crate::PrivilegeLevel::Ring2
    pub stack_pointer_2: u16,
    /// The stack [`SegmentSelector`] that should be loaded when a transfer occurs to
    /// [`PrivilegeLevel::Ring2`][plr2].
    ///
    /// [plr2]: crate::PrivilegeLevel::Ring2
    pub stack_segment_2: SegmentSelector,

    /// The value corresponding to the `ip` register.
    pub instruction_pointer: u16,
    /// The value corresponding to the [`Flags`] register.
    pub flags: u16,
    /// The value corresponding to the `ax` register.
    pub ax: u16,
    /// The value corresponding to the `cx` register.
    pub cx: u16,
    /// The value corresponding to the `dx` register.
    pub dx: u16,
    /// The value corresponding to the `bx` register.
    pub bx: u16,
    /// The value corresponding to the `sp` register.
    pub sp: u16,
    /// The value corresponding to the `bp` register.
    pub bp: u16,
    /// The value corresponding to the `si` register.
    pub si: u16,
    /// The value corresponding to the `di` register.
    pub di: u16,

    /// The [`SegmentSelector`] corresponding to the [`ES`][es] register.
    ///
    /// [es]: crate::registers::segmentation::ES
    pub es: SegmentSelector,
    /// The [`SegmentSelector`] corresponding to the [`CS`][cs] register.
    ///
    /// [cs]: crate::registers::segmentation::CS
    pub cs: SegmentSelector,
    /// The [`SegmentSelector`] corresponding to the [`SS`][ss] register.
    ///
    /// [ss]: crate::registers::segmentation::SS
    pub ss: SegmentSelector,
    /// The [`SegmentSelector`] corresponding to the [`DS`][ds] register.
    ///
    /// [ds]: crate::registers::segmentation::DS
    pub ds: SegmentSelector,

    /// The [`SegmentSelector`] for the tasks LDT.
    pub ldt: SegmentSelector,
}

impl TaskStateSegment16 {
    /// Creates a new [`TaskStateSegment16`] with every field set to zero.
    pub const fn new() -> Self {
        Self {
            previous_task_link: SegmentSelector::from_raw(0),

            stack_pointer_0: 0,
            stack_segment_0: SegmentSelector::from_raw(0),

            stack_pointer_1: 0,
            stack_segment_1: SegmentSelector::from_raw(0),

            stack_pointer_2: 0,
            stack_segment_2: SegmentSelector::from_raw(0),

            instruction_pointer: 0,
            flags: 0,
            ax: 0,
            cx: 0,
            dx: 0,
            bx: 0,
            sp: 0,
            bp: 0,
            si: 0,
            di: 0,

            es: SegmentSelector::from_raw(0),
            cs: SegmentSelector::from_raw(0),
            ss: SegmentSelector::from_raw(0),
            ds: SegmentSelector::from_raw(0),
            ldt: SegmentSelector::from_raw(0),
        }
    }
}

impl Default for TaskStateSegment16 {
    fn default() -> Self {
        Self::new()
    }
}

/// Processor state information needed to restore a 32-bit task.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TaskStateSegment32 {
    /// The [`SegmentSelector`] of the [`TaskStateSegment32`] of the previous task.
    pub previous_task_link: SegmentSelector,
    /// 0th reserved region.
    _reserved_0: u16,

    /// THe stack pointer that should be loaded when a transfer occurs to
    /// [`PrivilegeLevel::Ring0`][plr0]
    ///
    /// [plr0]: crate::PrivilegeLevel::Ring0
    pub stack_pointer_0: u32,
    /// The stack [`SegmentSelector`] that should be loaded when a transfer occurs to
    /// [`PrivilegeLevel::Ring0`][plr0].
    ///
    /// [plr0]: crate::PrivilegeLevel::Ring0
    pub stack_segment_0: SegmentSelector,
    /// 1st reserved region.
    _reserved_1: u16,

    /// THe stack pointer that should be loaded when a transfer occurs to
    /// [`PrivilegeLevel::Ring1`][plr1]
    ///
    /// [plr1]: crate::PrivilegeLevel::Ring1
    pub stack_pointer_1: u32,
    /// The stack [`SegmentSelector`] that should be loaded when a transfer occurs to
    /// [`PrivilegeLevel::Ring1`][plr1].
    ///
    /// [plr1]: crate::PrivilegeLevel::Ring1
    pub stack_segment_1: SegmentSelector,
    /// 2nd reserved region.
    _reserved_2: u16,

    /// THe stack pointer that should be loaded when a transfer occurs to
    /// [`PrivilegeLevel::Ring2`][plr2]
    ///
    /// [plr2]: crate::PrivilegeLevel::Ring2
    pub stack_pointer_2: u32,
    /// The stack [`SegmentSelector`] that should be loaded when a transfer occurs to
    /// [`PrivilegeLevel::Ring2`][plr2].
    ///
    /// [plr2]: crate::PrivilegeLevel::Ring2
    pub stack_segment_2: SegmentSelector,
    /// 3rd reserved region.
    _reserved_3: u16,

    /// The physical address of the top level page table for this task.
    pub cr3: u32,

    /// The value corresponding to the `eip` register.
    pub instruction_pointer: u32,
    /// The value corresponding to the [`Flags`] register.
    pub flags: u32,
    /// The value corresponding to the `eax` register.
    pub eax: u32,
    /// The value corresponding to the `ecx` register.
    pub ecx: u32,
    /// The value corresponding to the `edx` register.
    pub edx: u32,
    /// The value corresponding to the `ebx` register.
    pub ebx: u32,
    /// The value corresponding to the `esp` register.
    pub esp: u32,
    /// The value corresponding to the `ebp` register.
    pub ebp: u32,
    /// The value corresponding to the `esi` register.
    pub esi: u32,
    /// The value corresponding to the `edi` register.
    pub edi: u32,

    /// The [`SegmentSelector`] corresponding to the [`ES`][es] register.
    ///
    /// [es]: crate::registers::segmentation::ES
    pub es: SegmentSelector,
    /// 4th reserved region.
    _reserved_4: u16,

    /// The [`SegmentSelector`] corresponding to the [`CS`][cs] register.
    ///
    /// [cs]: crate::registers::segmentation::CS
    pub cs: SegmentSelector,
    /// 5th reserved region.
    _reserved_5: u16,

    /// The [`SegmentSelector`] corresponding to the [`SS`][ss] register.
    ///
    /// [ss]: crate::registers::segmentation::SS
    pub ss: SegmentSelector,
    /// 6th reserved region.
    _reserved_6: u16,

    /// The [`SegmentSelector`] corresponding to the [`DS`][ds] register.
    ///
    /// [ds]: crate::registers::segmentation::DS
    pub ds: SegmentSelector,
    /// 7th reserved region.
    _reserved_7: u16,

    /// The [`SegmentSelector`] corresponding to the [`FS`][fs] register.
    ///
    /// [fs]: crate::registers::segmentation::FS
    pub fs: SegmentSelector,
    /// 8th reserved region.
    _reserved_8: u16,

    /// The [`SegmentSelector`] corresponding to the [`GS`][gs] register.
    ///
    /// [gs]: crate::registers::segmentation::GS
    pub gs: SegmentSelector,
    /// 9th reserved region.
    _reserved_9: u16,

    /// The [`SegmentSelector`] for the task's LDT.
    pub ldt: SegmentSelector,
    /// 10th reserved region.
    _reserved_10: u16,

    /// Bit 0 of this field determines whether the processor should raise a debug exception when a
    /// task switch to this task occurs.
    pub trap_reserved: u16,

    /// A 16-bit offset from the base of this [`TaskStateSegment32`] to the I/O permission bit mpa
    /// and interrupt redirection bitmap.
    pub io_map_base_address: u16,
}

impl TaskStateSegment32 {
    /// Creates a new [`TaskStateSegment32`] with every field set to zero.
    pub const fn new() -> Self {
        Self {
            previous_task_link: SegmentSelector::from_raw(0),
            _reserved_0: 0,

            stack_pointer_0: 0,
            stack_segment_0: SegmentSelector::from_raw(0),
            _reserved_1: 0,

            stack_pointer_1: 0,
            stack_segment_1: SegmentSelector::from_raw(0),
            _reserved_2: 0,

            stack_pointer_2: 0,
            stack_segment_2: SegmentSelector::from_raw(0),
            _reserved_3: 0,

            cr3: 0,

            instruction_pointer: 0,
            flags: 0,
            eax: 0,
            ecx: 0,
            edx: 0,
            ebx: 0,
            esp: 0,
            ebp: 0,
            esi: 0,
            edi: 0,

            es: SegmentSelector::from_raw(0),
            _reserved_4: 0,
            cs: SegmentSelector::from_raw(0),
            _reserved_5: 0,
            ss: SegmentSelector::from_raw(0),
            _reserved_6: 0,
            ds: SegmentSelector::from_raw(0),
            _reserved_7: 0,
            fs: SegmentSelector::from_raw(0),
            _reserved_8: 0,
            gs: SegmentSelector::from_raw(0),
            _reserved_9: 0,

            ldt: SegmentSelector::from_raw(0),
            _reserved_10: 0,

            trap_reserved: 0,

            io_map_base_address: 0,
        }
    }
}

impl Default for TaskStateSegment32 {
    fn default() -> Self {
        Self::new()
    }
}

/// A 64-bit task state segment.
///
/// This contains stacks to which to switch when entering a [`PrivilegeLevel`][pl], stacks to which to
/// switch upon an interrupt, and a I/O permsission bitmap.
///
/// [pl]: crate::PrivilegeLevel
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct TaskStateSegment64 {
    /// The 0th reserved region.
    _reserved_0: u32,

    /// Lower 32 bits of the stack pointer for [`PrivilegeLevel::Ring0`][pl].
    ///
    /// [pl]: crate::PrivilegeLevel::Ring0
    pub stack_pointer_0_low: u32,
    /// Upper 32 bits of the stack pointer for [`PrivilegeLevel::Ring0`][pl].
    ///
    /// [pl]: crate::PrivilegeLevel::Ring0
    pub stack_pointer_0_high: u32,

    /// Lower 32 bits of the stack pointer for [`PrivilegeLevel::Ring1`][pl].
    ///
    /// [pl]: crate::PrivilegeLevel::Ring1
    pub stack_pointer_1_low: u32,
    /// Upper 32 bits of the stack pointer for [`PrivilegeLevel::Ring1`][pl].
    ///
    /// [pl]: crate::PrivilegeLevel::Ring1
    pub stack_pointer_1_high: u32,

    /// Lower 32 bits of the stack pointer for [`PrivilegeLevel::Ring2`][pl].
    ///
    /// [pl]: crate::PrivilegeLevel::Ring2
    pub stack_pointer_2_low: u32,
    /// Upper 32 bits of the stack pointer for [`PrivilegeLevel::Ring2`][pl].
    ///
    /// [pl]: crate::PrivilegeLevel::Ring2
    pub stack_pointer_2_high: u32,

    /// The 1st reserved region.
    _reserved_1: [u32; 2],

    /// Lower 32 bits of interrupt stack 1.
    pub ist1_low: u32,
    /// Upper 32 bits of interrupt stack 1.
    pub ist1_high: u32,

    /// Lower 32 bits of interrupt stack 2.
    pub ist2_low: u32,
    /// Upper 32 bits of interrupt stack 2.
    pub ist2_high: u32,

    /// Lower 32 bits of interrupt stack 3.
    pub ist3_low: u32,
    /// Upper 32 bits of interrupt stack 3.
    pub ist3_high: u32,

    /// Lower 32 bits of interrupt stack 4.
    pub ist4_low: u32,
    /// Upper 32 bits of interrupt stack 4.
    pub ist4_high: u32,

    /// Lower 32 bits of interrupt stack 5.
    pub ist5_low: u32,
    /// Upper 32 bits of interrupt stack 5.
    pub ist5_high: u32,

    /// Lower 32 bits of interrupt stack 6.
    pub ist6_low: u32,
    /// Upper 32 bits of interrupt stack 6.
    pub ist6_high: u32,

    /// Lower 32 bits of interrupt stack 7.
    pub ist7_low: u32,
    /// Upper 32 bits of interrupt stack 7.
    pub ist7_high: u32,

    /// The 2nd reserved region.
    _reserved_2: [u32; 2],
    /// The 3rd reserved region.
    _reserved_3: u16,

    /// Offset to the I/O permission bit map from the base of this [`TaskStateSegment64`].
    pub io_map_base: u16,
}

impl TaskStateSegment64 {
    /// Creates new [`TaskStateSegment64`] with all fields set to zero.
    pub const fn new() -> Self {
        Self {
            _reserved_0: 0,
            stack_pointer_0_low: 0,
            stack_pointer_0_high: 0,
            stack_pointer_1_low: 0,
            stack_pointer_1_high: 0,
            stack_pointer_2_low: 0,
            stack_pointer_2_high: 0,
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
}
