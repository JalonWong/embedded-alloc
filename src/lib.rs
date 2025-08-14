#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(feature = "allocator_api", feature(allocator_api, alloc_layout_extra))]
#![warn(missing_docs)]

#[cfg(feature = "llff")]
mod llff;
#[cfg(feature = "tlsf")]
mod tlsf;

#[cfg(feature = "llff")]
pub use llff::Heap as LlffHeap;
#[cfg(feature = "tlsf")]
pub use tlsf::Heap as TlsfHeap;

/// Init a heap with a static memory region.
#[macro_export]
macro_rules! init {
    ($heap:ident, $size:expr) => {
        static mut HEAP_MEM: [core::mem::MaybeUninit<u8>; $size] =
            [core::mem::MaybeUninit::uninit(); $size];
        #[allow(static_mut_refs)]
        unsafe {
            $heap.init(&raw mut HEAP_MEM as usize, $size)
        }
    };
}
