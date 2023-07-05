use core::mem::size_of;
use core::ops::IndexMut;
use core::ptr::NonNull;
use core::slice::{self, from_raw_parts_mut};

use crate::boot::{self, BootAlloc};
use crate::mm::util::*;
use crate::sync::singleton::Singleton;

use super::metapage::MetaPage;

static META_PAGE_TABLE: Singleton<&'static mut [MetaPage]> = Singleton::uninit();

pub unsafe fn alloc_meta_page_table(bootalloc: &mut BootAlloc) -> NonNull<[MetaPage]> {
	let page_count = unsafe { boot::MEM_INFO.end_pfn };

	let ptr = bootalloc.alloc_n::<MetaPage>(page_count);
	let ptr = slice::from_raw_parts_mut(ptr.as_ptr(), page_count);

	NonNull::new_unchecked(ptr)
}

pub unsafe fn init(table: NonNull<[MetaPage]>) {
	let base_ptr = table.as_ptr() as *mut MetaPage;
	let count = table.len();

	for entry in (0..count).map(|x| base_ptr.add(x)) {
		MetaPage::construct_at(entry);
	}

	META_PAGE_TABLE.write(from_raw_parts_mut(base_ptr, count));
}

pub fn meta_to_ptr(page: NonNull<MetaPage>) -> NonNull<u8> {
	let index = meta_to_index(page);

	return unsafe { NonNull::new_unchecked(phys_to_virt(pfn_to_addr(index)) as *mut u8) };
}

pub fn ptr_to_meta(ptr: NonNull<u8>) -> NonNull<MetaPage> {
	let index = addr_to_pfn(virt_to_phys(ptr.as_ptr() as usize));

	return index_to_meta(index);
}

pub fn meta_to_index(page: NonNull<MetaPage>) -> usize {
	let addr = page.as_ptr() as usize;
	let base = META_PAGE_TABLE.lock().as_ptr() as usize;

	(addr - base) / size_of::<MetaPage>()
}

pub fn index_to_meta(index: usize) -> NonNull<MetaPage> {
	NonNull::from(META_PAGE_TABLE.lock().index_mut(index))
}
