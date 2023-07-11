mod address_space;
mod address_tree;
mod kmap;
mod test;
mod virtual_allocator;

use core::alloc::{AllocError, Allocator, Layout};
use core::ptr::NonNull;
use core::slice;

use address_space::*;
use address_tree::*;
use virtual_allocator::*;

use crate::mm::page::{map_page, unmap_page, PageFlag};
use crate::mm::{constant::*, util::*};

pub use address_space::AddressSpace;

pub fn allocate(layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
	VMALLOC.allocate(layout)
}

pub fn deallocate(ptr: NonNull<u8>, layout: Layout) {
	unsafe { VMALLOC.deallocate(ptr, layout) };
}

pub fn lookup_size(ptr: NonNull<u8>) -> usize {
	VMALLOC.size(ptr)
}

pub fn init() {
	VMALLOC.init(addr_to_pfn(VMALLOC_OFFSET)..addr_to_pfn(KMAP_OFFSET));
}

pub fn io_allocate(paddr: usize, count: usize) -> Result<NonNull<[u8]>, AllocError> {
	let vaddr = ADDRESS_TREE.lock().alloc(count).ok_or_else(|| AllocError)?;

	for (vaddr, paddr) in (0..count).map(|x| (vaddr + x * PAGE_SIZE, paddr + x * PAGE_SIZE)) {
		map_page(
			vaddr,
			paddr,
			PageFlag::Present
				| PageFlag::Global
				| PageFlag::Write
				| PageFlag::PAT | PageFlag::PCD
				| PageFlag::PWT,
		)?;
	}

	unsafe {
		Ok(NonNull::from(slice::from_raw_parts(
			vaddr as *mut u8,
			rank_to_size(count),
		)))
	}
}

pub fn io_deallocate(vaddr: usize, count: usize) {
	ADDRESS_TREE.lock().dealloc(vaddr, count);

	for vaddr in (0..count).map(|x| vaddr + x * PAGE_SIZE) {
		let _ = unmap_page(vaddr);
	}
}
