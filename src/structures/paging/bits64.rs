//! Definitions and interfaces to interact with 4-level and 5-level paging.
//!
//! 4-level and 5-level paging structures are only used in `x86_64` long mode.

use core::marker::PhantomData;

/// Representation of a page table.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PageTable<L: PageMapLevel>([PageMapEntry<L, Unclassified>; 512]);

impl<L: PageMapLevel> PageTable<L> {
    /// Creates an empty [`PageTable`].
    pub const fn new() -> Self {
        Self(
            [const {
                PageMapEntry {
                    value: 0,
                    phantom: PhantomData,
                }
            }; 512],
        )
    }

    /// Gets the [`PageMapEntry`] located at `index`.
    ///
    /// Returns [`None`] if `index` is out of bounds.
    pub const fn get(&self, index: usize) -> Option<PageMapEntry<L, Unclassified>> {
        #[allow(clippy::nonminimal_bool)]
        if !(index < self.0.len()) {
            return None;
        }

        Some(self.0[index])
    }

    /// Sets the [`PageMapEntry`] located at `index` to `entry`.
    ///
    /// # Errors
    /// Return [`Err`] if `index` is out of bounds.
    pub const fn set(
        &mut self,
        index: usize,
        entry: PageMapEntry<L, Unclassified>,
    ) -> Result<(), PageMapEntry<L, Unclassified>> {
        #[allow(clippy::nonminimal_bool)]
        if !(index < self.0.len()) {
            return Err(entry);
        }

        self.0[index] = entry;
        Ok(())
    }
}

impl<L: PageMapLevel> Default for PageTable<L> {
    fn default() -> Self {
        Self::new()
    }
}

/// A 64-bit page table entry.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PageMapEntry<L: PageMapLevel, S: PageMapEntryState> {
    /// The underlying value of the [`PageMapEntry`].
    value: u64,
    /// Phantom data used to ensure type safety.
    phantom: PhantomData<(L, S)>,
}

impl<L: PageMapLevel> PageMapEntry<L, Unclassified> {
    /// Creates a new [`PageMapEntry`] that is unclassified.
    pub const fn new() -> Self {
        Self {
            value: 0,
            phantom: PhantomData,
        }
    }
}

impl<L: PageMapLevel, S: PageMapEntryState> PageMapEntry<L, S> {
    /// Returns this [`PageMapEntry`] as a [`PageMapEntry<L, Unclassified>`].
    pub const fn unclassified(self) -> PageMapEntry<L, Unclassified> {
        PageMapEntry {
            value: self.value,
            phantom: PhantomData,
        }
    }

    /// Returns [`PageMapEntry<L, Present>`] if the [`PageMapEntry`] is present; otherwise, this
    /// function return [`None`].
    pub const fn present(self) -> Option<PageMapEntry<L, Present>> {
        #[allow(clippy::nonminimal_bool)]
        if !(self.value & 0b1 == 0b1) {
            return None;
        }

        Some(PageMapEntry {
            value: self.value,
            phantom: PhantomData,
        })
    }

    /// Sets the present bit in this [`PageMapEntry`].
    pub const fn set_present(self) -> PageMapEntry<L, Present> {
        PageMapEntry {
            value: self.value | 0b1,
            phantom: PhantomData,
        }
    }

    /// Clears the present bit in this [`PageMapEntry`].
    pub const fn clear_present(self) -> PageMapEntry<L, Unclassified> {
        PageMapEntry {
            value: self.value & !0b1,
            phantom: PhantomData,
        }
    }
}

impl<L: PageMapLevel, S: PageMapEntryPresent> PageMapEntry<L, S> {
    /// Returns `true` if the region of memory controlled by this [`PageMapEntry`] is writable.
    pub const fn writable(self) -> bool {
        self.value & (1 << 1) == (1 << 1)
    }

    /// Sets whether the region of memory controlled by this [`PageMapEntry`] should be writable.
    pub const fn set_writable(mut self, writable: bool) -> Self {
        self.value = (self.value & !(1 << 1)) | ((writable as u64) << 1);
        self
    }

    /// Returns `true` if the region of memory controlled by this [`PageMapEntry`] is not
    /// accessible to userspace.
    pub const fn user(self) -> bool {
        self.value & (1 << 2) == (1 << 2)
    }

    /// Sets whether the region of memory controlled by this [`PageMapEntry`] should be accessible
    /// to userspace.
    pub const fn set_user(mut self, accessible: bool) -> Self {
        self.value = (self.value & !(1 << 2)) | ((accessible as u64) << 2);
        self
    }

    /// Returns `true` if the bit is set.
    ///
    /// This bit helps determine the memory type used to access the item pointed to by this
    /// [`PageMapEntry`].
    pub const fn write_through(self) -> bool {
        self.value & (1 << 3) == (1 << 3)
    }

    /// Sets the write through bit.
    ///
    /// This bit helps determine the memory type used to access the item pointed to by this
    /// [`PageMapEntry`].
    pub const fn set_write_through(mut self, write_through: bool) -> Self {
        self.value = (self.value & !(1 << 3)) | ((write_through as u64) << 3);
        self
    }

    /// Returns `true` if the bit is set.
    ///
    /// This bit helps determine the memory type used to access the item pointed to by this
    /// [`PageMapEntry`].
    pub const fn cache_disable(self) -> bool {
        self.value & (1 << 4) == (1 << 4)
    }

    /// Sets the write through bit.
    ///
    /// This bit helps determine the memory type used to access the item pointed to by this
    /// [`PageMapEntry`].
    pub const fn set_cache_disable(mut self, cache_disable: bool) -> Self {
        self.value = (self.value & !(1 << 4)) | ((cache_disable as u64) << 4);
        self
    }

    /// Returns `true` if the region of memory controlled by this [`PageMapEntry`] has been
    /// accessed.
    pub const fn accessed(self) -> bool {
        self.value & (1 << 5) == (1 << 5)
    }

    /// Sets whether the region of memory controlled by this [`PageMapEntry`] should be marked as
    /// having been accessed.
    pub const fn set_accessed(mut self, accessed: bool) -> Self {
        self.value = (self.value & !(1 << 5)) | ((accessed as u64) << 5);
        self
    }

    /// Returns `true` if HLAT paging should restart with ordinary paging when translation
    /// encounters this [`PageMapEntry`].
    pub const fn restart(self) -> bool {
        self.value & (1 << 11) == (1 << 11)
    }

    /// Sets whether HLAT paging should restart with ordinary paging when translation encounters
    /// this [`PageMapEntry`].
    pub const fn set_restart(mut self, restart: bool) -> Self {
        self.value = (self.value & !(1 << 11)) | ((restart as u64) << 11);
        self
    }

    /// Returns `true` if the region of memory controlled by this [`PageMapEntry`] cannot be
    /// executed.
    pub const fn no_execute(self) -> bool {
        self.value & (1 << 63) == (1 << 63)
    }

    /// Sets whether the region of memory controlled by this [`PageMapEntry`] should be not
    /// executable.
    pub const fn set_no_execute(mut self, no_execute: bool) -> Self {
        self.value = (self.value & !(1 << 63)) | ((no_execute as u64) << 63);
        self
    }
}

impl<L: LeafSupport, S: PageMapEntryPresent> PageMapEntry<L, S> {
    /// Sets the [`PageMapEntry`] to be a leaf entry.
    pub const fn set_leaf(self, leaf_address: u64) -> PageMapEntry<L, Leaf> {
        PageMapEntry {
            value: (self.value & !L::ADDRESS_MASK) | (leaf_address & L::ADDRESS_MASK),
            phantom: PhantomData,
        }
    }
}

impl<L: BranchSupport, S: PageMapEntryPresent> PageMapEntry<L, S> {
    /// Sets the [`PageMapEntry`] to be a branch entry.
    pub const fn set_branch(self, branch_address: u64) -> PageMapEntry<L, Branch> {
        PageMapEntry {
            value: (self.value & !0x000F_FFFF_FFFF_F000) | (branch_address & 0x000F_FFFF_FFFF_F000),
            phantom: PhantomData,
        }
    }
}

impl<L: UnconditionalLeafSupport, S: PageMapEntryPresent> PageMapEntry<L, S> {
    /// Returns this [`PageMapEntry`] as a [`PageMapEntry<L, Leaf>`].
    pub const fn leaf(self) -> PageMapEntry<L, Leaf> {
        PageMapEntry {
            value: self.value,
            phantom: PhantomData,
        }
    }
}

impl<L: UnconditionalBranchSupport, S: PageMapEntryPresent> PageMapEntry<L, S> {
    /// Returns this [`PageMapEntry`] as a [`PageMapEntry<L, Branch>`].
    pub const fn branch(self) -> PageMapEntry<L, Branch> {
        PageMapEntry {
            value: self.value,
            phantom: PhantomData,
        }
    }
}

impl<L: BranchLeafSupport, S: PageMapEntryPresent> PageMapEntry<L, S> {
    /// Returns [`PageMapEntry<L, Leaf>`] if this [`PageMapEntry`] is a leaf entry; otherwise,
    /// this function returns [`None`].
    pub const fn leaf_opt(self) -> Option<PageMapEntry<L, Leaf>> {
        if !self.is_leaf() {
            return None;
        }

        Some(PageMapEntry {
            value: self.value,
            phantom: PhantomData,
        })
    }

    /// Returns [`PageMapEntry<L, Branch>`] if this [`PageMapEntry`] is a branch entry; otherwise,
    /// this function returns [`None`].
    pub const fn branch_opt(self) -> Option<PageMapEntry<L, Branch>> {
        if self.is_leaf() {
            return None;
        }

        Some(PageMapEntry {
            value: self.value,
            phantom: PhantomData,
        })
    }

    /// Returns `true` if this [`PageMapEntry`] is a leaf entry.
    const fn is_leaf(&self) -> bool {
        self.value & (1 << 7) == (1 << 7)
    }
}

impl<L: LeafSupport> PageMapEntry<L, Leaf> {
    /// Returns `true` if the region of memory controlled by this [`PageMapEntry`] has been written
    /// to.
    pub const fn dirty(self) -> bool {
        self.value & (1 << 6) == (1 << 6)
    }

    /// Sets whether the region of memory controlled by this [`PageMapEntry`] has been written to.
    pub const fn set_dirty(mut self, dirty: bool) -> Self {
        self.value = (self.value & !(1 << 6)) | ((dirty as u64) << 6);
        self
    }

    /// Returns `true` if the region of memory controlled by this [`PageMapEntry`] should be
    /// translated globally.
    pub const fn global(self) -> bool {
        self.value & (1 << 8) == (1 << 8)
    }

    /// Sets whether the region of memory controlled by this [`PageMapEntry`] should be translated
    /// globally.
    pub const fn set_global(mut self, global: bool) -> Self {
        self.value = (self.value & !(1 << 8)) | ((global as u64) << 8);
        self
    }

    /// Returns `true` if the bit is set.
    ///
    /// This bit helps determine the memory type used to access the item pointed to by this
    /// [`PageMapEntry`].
    pub const fn pat(self) -> bool {
        self.value & (1 << L::PAT_BIT_POS) == (1 << L::PAT_BIT_POS)
    }

    /// Sets the pat bit.
    ///
    /// This bit helps determine the memory type used to access the item pointed to by this
    /// [`PageMapEntry`].
    pub const fn set_pat(mut self, pat: bool) -> Self {
        self.value = (self.value & !(1 << L::PAT_BIT_POS)) | ((pat as u64) << L::PAT_BIT_POS);
        self
    }

    /// Returns the base address of the region of memory controlled by this [`PageMapEntry`].
    pub const fn frame(self) -> u64 {
        self.value & L::ADDRESS_MASK
    }
}

impl<L: BranchSupport> PageMapEntry<L, Branch> {
    /// Returns the base address of the next level of the page table hierarchy.
    pub const fn frame(self) -> u64 {
        self.value & 0x000F_FFFF_FFFF_F000
    }
}

impl<L: PageMapLevel> Default for PageMapEntry<L, Unclassified> {
    fn default() -> Self {
        Self::new()
    }
}

/// The lowest level of the [`PageTable`] hierarchy.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Pml1e;

/// The second level of the [`PageTable`] hierarchy.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Pml2e;

/// The third lowest level of the [`PageTable`] hierarchy.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Pml3e;

/// The highest or second highest level of the [`PageTable`] hierarchy.
///
/// Its place depends on whether 5-level paging has been enabled.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Pml4e;

/// The highest level of the [`PageTable`] hierarchy.
///
/// This may not be enabled, as 4-level paging is far more common.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Pml5e;

/// Marker trait indicating that the implementer is a valid page map level.
pub trait PageMapLevel: Copy + private::PageMapLevelSealed {}
impl PageMapLevel for Pml1e {}
impl PageMapLevel for Pml2e {}
impl PageMapLevel for Pml3e {}
impl PageMapLevel for Pml4e {}
impl PageMapLevel for Pml5e {}

/// Marker trait indicating that the [`PageMapEntry`]s of that [`PageMapLevel`] could be leaf
/// entries.
pub trait LeafSupport: PageMapLevel {
    /// Position of the PAT bit.
    const PAT_BIT_POS: u8;

    /// Bitmask to extract the address of the frames this [`PageMapEntry`] controls.
    const ADDRESS_MASK: u64;
}
impl LeafSupport for Pml1e {
    const PAT_BIT_POS: u8 = 7;
    const ADDRESS_MASK: u64 = 0x000F_FFFF_FFFF_F000;
}
impl LeafSupport for Pml2e {
    const PAT_BIT_POS: u8 = 12;
    const ADDRESS_MASK: u64 = 0x000F_FFFF_FFE0_0000;
}
impl LeafSupport for Pml3e {
    const PAT_BIT_POS: u8 = 12;
    const ADDRESS_MASK: u64 = 0x000F_FFFF_C000_0000;
}

/// Marker trait indicating that the [`PageMapEntry`]s of that [`PageMapLevel`] could be branch
/// entries.
pub trait BranchSupport: PageMapLevel {}
impl BranchSupport for Pml5e {}
impl BranchSupport for Pml4e {}
impl BranchSupport for Pml3e {}
impl BranchSupport for Pml2e {}

/// Marker trait that indicates that the [`PageMapEntry`]s of that [`PageMapLevel`]
/// are could be either branch or leaf entries.
pub trait BranchLeafSupport: LeafSupport + BranchSupport {}
impl BranchLeafSupport for Pml2e {}
impl BranchLeafSupport for Pml3e {}

/// Marker trait that indicates that the [`PageMapEntry`]s of that [`PageMapLevel`]
/// are unconditionally leaf entries.
pub trait UnconditionalLeafSupport: LeafSupport {}
impl UnconditionalLeafSupport for Pml1e {}

/// Marker trait that indicates that the [`PageMapEntry`]s of that [`PageMapLevel`]
/// are unconditionally branch entries.
pub trait UnconditionalBranchSupport: BranchSupport {}
impl UnconditionalBranchSupport for Pml5e {}
impl UnconditionalBranchSupport for Pml4e {}

/// Marker struct that indicates that the [`PageMapEntry`] has not been classified.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Unclassified;

/// Marker struct that indicates that the [`PageMapEntry`] is present.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Present;

/// Marker struct that indicates that the [`PageMapEntry`] is a leaf entry.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Leaf;

/// Marker struct that indicates that the [`PageMapEntry`] is a branch entry.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Branch;

/// Marker trait that indicates that the implementer is a valid state of a [`PageMapEntry`].
pub trait PageMapEntryState: Copy + private::PageMapEntryStateSealed {}
impl PageMapEntryState for Unclassified {}
impl PageMapEntryState for Present {}
impl PageMapEntryState for Leaf {}
impl PageMapEntryState for Branch {}

/// Marker trait that indicates that the [`PageMapEntryState`] is a subset of [`Present`].
pub trait PageMapEntryPresent: PageMapEntryState {}
impl PageMapEntryPresent for Present {}
impl PageMapEntryPresent for Leaf {}
impl PageMapEntryPresent for Branch {}

mod private {
    //! Module used to seal the various traits used to implement the 64-bit paging abstraction.

    use crate::structures::paging::bits64::{
        Branch, Leaf, Pml1e, Pml2e, Pml3e, Pml4e, Pml5e, Present, Unclassified,
    };

    /// Marker trait used to seal [`PageMapLevel`].
    pub trait PageMapLevelSealed {}

    impl PageMapLevelSealed for Pml1e {}
    impl PageMapLevelSealed for Pml2e {}
    impl PageMapLevelSealed for Pml3e {}
    impl PageMapLevelSealed for Pml4e {}
    impl PageMapLevelSealed for Pml5e {}

    /// Marker trait used to seal [`PageMapEntryState`].
    pub trait PageMapEntryStateSealed {}

    impl PageMapEntryStateSealed for Unclassified {}
    impl PageMapEntryStateSealed for Present {}
    impl PageMapEntryStateSealed for Leaf {}
    impl PageMapEntryStateSealed for Branch {}
}

#[cfg(test)]
mod tests {
    use core::marker::PhantomData;

    use super::{PageMapEntry, PageTable, Pml5e, Unclassified};

    #[test]
    fn pml5e() {
        let entry = PageMapEntry::<Pml5e, Unclassified> {
            value: 0b0000_0010_1011,
            phantom: PhantomData,
        };
        let entry = entry.present().unwrap();

        assert!(entry.writable());
        assert!(!entry.user());
        assert!(entry.write_through());
        assert!(!entry.cache_disable());
        assert!(entry.accessed());

        let entry = entry.branch();
        assert_eq!(entry.frame(), 0)
    }

    #[test]
    fn page_table() {
        let mut table = PageTable::<Pml5e>::new();
        let entry = table.get(0).unwrap();

        let entry = entry
            .set_present()
            .set_user(false)
            .set_restart(false)
            .set_writable(false)
            .set_accessed(false)
            .set_no_execute(true)
            .set_write_through(false)
            .set_cache_disable(false)
            .set_branch(0xF0_0000);

        table.set(0, entry.unclassified()).unwrap();
    }
}
