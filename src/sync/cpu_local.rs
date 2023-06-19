use core::{cell::UnsafeCell, mem::MaybeUninit};

use crate::{config::NR_CPUS, smp::smp_id};

pub struct CpuLocal<T> {
	data: UnsafeCell<MaybeUninit<[T; NR_CPUS]>>,
}

unsafe impl<T> Sync for CpuLocal<T> {}

impl<T> CpuLocal<T> {
	pub const fn zeroed() -> Self {
		Self {
			data: UnsafeCell::new(MaybeUninit::zeroed()),
		}
	}

	pub fn get_mut(&self) -> LocalValue<'_, T> {
		let arr = self.arr_mut();

		LocalValue::new(&mut arr[smp_id()])
	}

	fn arr_mut<'l>(&self) -> &'l mut [T; NR_CPUS] {
		unsafe { self.data.get().as_mut::<'l>().unwrap().assume_init_mut() }
	}
}

pub struct LocalValue<'l, T> {
	value: &'l mut T,
}

impl<'l, T> LocalValue<'l, T> {
	fn new(value: &'l mut T) -> Self {
		unsafe { core::arch::asm!("cli") };
		LocalValue { value }
	}
}

impl<'l, T> Drop for LocalValue<'l, T> {
	fn drop(&mut self) {
		unsafe { core::arch::asm!("sti") };
	}
}

#[cfg(disable)]
mod test {
	use crate::pr_info;
	use kfs_macro::ktest;

	use super::*;

	#[derive(Debug)]
	struct A {
		a: usize,
		b: usize,
	}

	static AA: CpuLocal<A> = CpuLocal::zeroed();

	#[ktest(dev)]
	fn test() {
		let mut a = AA.get_mut();
		let mut b = AA.get_mut();

		b.a = 2;
		a.a = 1;

		let c = AA.get_mut();

		pr_info!("c.a: {}", c.a);
		pr_info!("c.b: {}", c.b);
	}
}
