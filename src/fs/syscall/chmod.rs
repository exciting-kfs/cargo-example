use crate::fs::path::Path;
use crate::fs::vfs::{lookup_entry, Permission};
use crate::{process::task::CURRENT, syscall::errno::Errno};

use super::utils::verify_path;

pub fn sys_chmod(path: usize, perm: u32) -> Result<usize, Errno> {
	let current = unsafe { CURRENT.get_ref() };

	let path = verify_path(path, current)?;
	let path = Path::new(path);

	let entry = lookup_entry(path, current)?;

	entry
		.chmod(Permission::from_bits_truncate(perm), current)
		.map(|_| 0)
}
