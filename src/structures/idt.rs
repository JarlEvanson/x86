//! Definitions and interfaces to interact with `x86` and `x86_64` interrupt descriptor tables.

use core::fmt;

use crate::registers::{
    flags::{ArchitectureExt, Flags},
    segmentation::SegmentSelector,
};

/// The interrupt stack frame pushed by the CPU whenever an exception or interrupt occurs.
#[repr(C)]
pub struct InterruptStackFrame<A: ArchitectureExt> {
    /// Pointer to the instruction that should be executed when the interrupt handler returns.
    pub instruction_pointer: A::GeneralRegister,
    /// The code [`SegmentSelector`] at the time of the interrupt.
    pub code_segment: SegmentSelector,
    /// The cpu [`Flags`] at the time of the interrupt.
    pub cpu_flags: Flags<A>,
    /// The stack pointer at the time of the interrupt.
    pub stack_pointer: A::GeneralRegister,
    /// The stack [`SegmentSelector`] at the time of the interrupt.
    pub stack_segment: SegmentSelector,
}

impl<A: ArchitectureExt> Clone for InterruptStackFrame<A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<A: ArchitectureExt> Copy for InterruptStackFrame<A> {}

impl<A: ArchitectureExt> fmt::Debug for InterruptStackFrame<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("InterruptStackFrame");

        debug_struct.field("instruction_pointer", &self.instruction_pointer);
        debug_struct.field("code_segment", &self.code_segment);
        debug_struct.field("cpu_flags", &self.cpu_flags);
        debug_struct.field("stack_pointer", &self.stack_pointer);
        debug_struct.field("stack_segment", &self.stack_segment);

        debug_struct.finish()
    }
}

impl<A: ArchitectureExt> core::hash::Hash for InterruptStackFrame<A> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.instruction_pointer.hash(state);
        self.code_segment.hash(state);
        self.cpu_flags.hash(state);
        self.stack_pointer.hash(state);
        self.stack_segment.hash(state);
    }
}

impl<A: ArchitectureExt> PartialEq for InterruptStackFrame<A> {
    fn eq(&self, other: &Self) -> bool {
        self.instruction_pointer == other.instruction_pointer
            && self.code_segment == other.code_segment
            && self.cpu_flags == other.cpu_flags
            && self.stack_pointer == other.stack_pointer
            && self.stack_segment == other.stack_segment
    }
}

impl<A: ArchitectureExt> Eq for InterruptStackFrame<A> {}
