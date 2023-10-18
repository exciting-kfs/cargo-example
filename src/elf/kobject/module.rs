use alloc::collections::BTreeMap;
use alloc::sync::Arc;

use crate::{ptr::VirtPageBox, sync::Locked, syscall::errno::Errno};

#[macro_export]
macro_rules! kernel_module {
	{$($fields:tt)*} => {
		#[link_section = ".kfs"]
		#[used]
		pub static __MODULE: kernel::elf::kobject::KernelModuleInfo = kernel::elf::kobject::KernelModuleInfo {
			$($fields)*
		};
	};
}

pub struct KernelModuleInfo {
	pub name: &'static [u8],
	pub init: fn(this: Arc<KernelModule>) -> Result<(), Errno>,
	pub cleanup: Option<fn()>,
}
pub struct KernelModule {
	mem: VirtPageBox,
	info_offset: usize,
}

impl Drop for KernelModule {
	fn drop(&mut self) {
		if let Some(cleanup) = self.get_info().cleanup {
			(cleanup)()
		}
	}
}

static LOADED_MODULES: Locked<BTreeMap<&[u8], Arc<KernelModule>>> = Locked::new(BTreeMap::new());

impl KernelModule {
	pub fn new(mem: VirtPageBox, info_offset: usize) -> Arc<Self> {
		Arc::new(Self { mem, info_offset })
	}

	pub fn get_info(&self) -> &KernelModuleInfo {
		unsafe {
			&*((&self.mem.as_slice()[self.info_offset]) as *const u8).cast::<KernelModuleInfo>()
		}
	}
}

pub fn load_kernel_module<'a>(module: Arc<KernelModule>) -> Result<(), Errno> {
	let mut loaded_modules = LOADED_MODULES.lock();

	if let Some(_) = loaded_modules.get(module.get_info().name) {
		return Err(Errno::EEXIST);
	}

	(module.get_info().init)(module.clone())?;

	loaded_modules.insert(module.get_info().name, module);

	Ok(())
}

pub fn cleanup_kernel_module(name: &[u8]) -> Result<(), Errno> {
	let mut loaded_modules = LOADED_MODULES.lock();

	let module = loaded_modules.get(name).ok_or(Errno::ENOENT)?;

	if Arc::strong_count(module) != 1 {
		return Err(Errno::EBUSY);
	}

	if Arc::weak_count(module) != 0 {
		return Err(Errno::EBUSY);
	}

	loaded_modules.remove(name);

	Ok(())
}
