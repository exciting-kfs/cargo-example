use core::{
	cell::UnsafeCell,
	ops::{Deref, DerefMut},
	sync::atomic::{AtomicUsize, Ordering},
};

use super::lock::spinlock::SpinLock;

#[derive(Debug)]
pub struct LockRW<T> {
	write_lock: SpinLock,
	read_count: AtomicUsize,
	value: UnsafeCell<T>,
}

unsafe impl<T> Send for LockRW<T> {}
unsafe impl<T> Sync for LockRW<T> {}

impl<T> LockRW<T> {
	pub const fn new(value: T) -> Self {
		Self {
			write_lock: SpinLock::new(),
			read_count: AtomicUsize::new(0),
			value: UnsafeCell::new(value),
		}
	}

	pub fn read_lock(&self) -> ReadLockGuard<'_, T> {
		self.write_lock.lock();
		self.write_lock.unlock();
		self.read_count.fetch_add(1, Ordering::Relaxed);

		unsafe { ReadLockGuard::new(self) }
	}

	pub unsafe fn read_lock_manual(&self) -> &T {
		self.write_lock.lock();
		self.write_lock.unlock();
		self.read_count.fetch_add(1, Ordering::Relaxed);

		unsafe { &*self.value.get() }
	}

	pub unsafe fn read_unlock_manual(&self) {
		self.read_count.fetch_sub(1, Ordering::Relaxed);
	}

	pub fn write_lock(&self) -> WriteLockGuard<'_, T> {
		self.write_lock.lock();

		while self.read_count.load(Ordering::Relaxed) == 0 {}

		unsafe { WriteLockGuard::new(self) }
	}
}

pub struct ReadLockGuard<'lock, T> {
	lock: &'lock LockRW<T>,
}

impl<'lock, T> ReadLockGuard<'lock, T> {
	pub unsafe fn new(lock: &'lock LockRW<T>) -> Self {
		Self { lock }
	}
}

impl<'lock, T> Drop for ReadLockGuard<'lock, T> {
	fn drop(&mut self) {
		self.lock.read_count.fetch_sub(1, Ordering::Relaxed);
	}
}

impl<'lock, T> Deref for ReadLockGuard<'lock, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.lock.value.get() }
	}
}

pub struct WriteLockGuard<'lock, T> {
	lock: &'lock LockRW<T>,
}

impl<'lock, T> WriteLockGuard<'lock, T> {
	pub unsafe fn new(lock: &'lock LockRW<T>) -> Self {
		Self { lock }
	}
}

impl<'lock, T> Drop for WriteLockGuard<'lock, T> {
	fn drop(&mut self) {
		self.lock.write_lock.unlock();
	}
}

impl<'lock, T> Deref for WriteLockGuard<'lock, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.lock.value.get() }
	}
}

impl<'lock, T> DerefMut for WriteLockGuard<'lock, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *self.lock.value.get() }
	}
}
