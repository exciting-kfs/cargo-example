use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::cell::UnsafeCell;

use crate::kmem_cache_register;

use super::slub::ForSizeCache;
use super::slub::SizeCache;
use super::slub::alloc_pages_from_page_alloc;
use super::slub::dealloc_pages_to_page_alloc;
use super::util::bit_scan_reverse;

static mut SIZE64: SizeCache<'static, 64> = SizeCache::new();		// RANK 6
static mut SIZE128: SizeCache<'static, 128> = SizeCache::new();
static mut SIZE256: SizeCache<'static, 256> = SizeCache::new();
static mut SIZE512: SizeCache<'static, 512> = SizeCache::new();
static mut SIZE1024: SizeCache<'static, 1024> = SizeCache::new();
static mut SIZE2048: SizeCache<'static, 2048> = SizeCache::new();	// RANK 11

const RANK_MIN: usize = 6;
const RANK_END: usize = 12;

/// trait Allocator vs trait GlobalAlloc
///
/// Collections in std, these use [std::alloc::Global] by default that satisfies trait [core::alloc::Allocator].
/// To change [std::alloc::Global] to our custom allocator, We should use proc-macro [global_allocator].
/// proc-macro [global_allocator] requires trait [core::alloc::GlobalAlloc], not trait [core::alloc::Allocator].

#[global_allocator]
pub static G: GlobalAllocator = GlobalAllocator::new();

pub struct GlobalAllocator(UnsafeCell<bool>); // TODO Atomic?

unsafe impl Sync for GlobalAllocator {} // ?

impl GlobalAllocator {
	pub const fn new() -> Self {
		GlobalAllocator(UnsafeCell::new(false))
	}

	pub unsafe fn lazy_init(&self) {
		if ! *self.0.get() {
			kmem_cache_register!(SIZE2048);
			kmem_cache_register!(SIZE1024);
			kmem_cache_register!(SIZE512);
			kmem_cache_register!(SIZE256);
			kmem_cache_register!(SIZE128);
			kmem_cache_register!(SIZE64);
			(*self.0.get()) = true;
		}
	}
}

unsafe impl GlobalAlloc for GlobalAllocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		self.lazy_init();

		let rank = rank_of(layout);
		match rank.checked_sub(RANK_END) {
			None => get_allocator(rank).allocate(),
			Some(r) => match alloc_pages_from_page_alloc(1 << r) {
				Ok(ptr) => ptr.as_mut_ptr(),
				Err(_) => 0 as *mut u8
			}
		}
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		self.lazy_init();

		let rank = rank_of(layout);
		match rank.checked_sub(RANK_END) {
			None => get_allocator(rank).deallocate(ptr),
			Some(r) => dealloc_pages_to_page_alloc(ptr, 1 << r),
		}
	}
}

unsafe fn get_allocator<'a>(rank: usize) -> &'a mut dyn ForSizeCache {
	let caches: [&mut dyn ForSizeCache; 6] = [
		&mut SIZE64,
		&mut SIZE128,
		&mut SIZE256,
		&mut SIZE512,
		&mut SIZE1024,
		&mut SIZE2048
	];
	caches[rank - RANK_MIN]
}

fn rank_of(layout: Layout) -> usize {
	let size = layout.size();
	let align = layout.align();

	if size == 1 && align == 1 {
		return RANK_MIN;
	}

	let rank = match size > align {
		true => bit_scan_reverse(size - 1) + 1,
		false => bit_scan_reverse(align - 1) + 1,
	};

	RANK_MIN + rank.checked_sub(RANK_MIN).unwrap_or_default()
}

pub fn kmalloc(bytes: usize) -> &'static mut [u8] {

	unsafe {
		let layout = Layout::from_size_align_unchecked(bytes, core::mem::align_of::<u8>());
		core::slice::from_raw_parts_mut(
			G.alloc(layout),
			bytes
		)
	}
}

pub unsafe fn kfree(ptr: &mut [u8]) {
	let layout = Layout::from_size_align_unchecked(ptr.len(), core::mem::align_of::<u8>());
	G.dealloc(ptr.as_mut_ptr(), layout)
}

mod test {

	use kfs_macro::ktest;
	use alloc::{vec::Vec};
	use alloc::vec;

	use crate::{pr_info};

	#[ktest]
	fn test_alloc() {
		let mut v: Vec<usize> = vec![1, 2, 3];

		v.iter().for_each(|e| {
			pr_info!("{}", e);
		});

		for i in 0..100 {
			v.push(i);
		}
		drop(v);
	}

	#[ktest]
	fn test_kmalloc() {
	}
}
