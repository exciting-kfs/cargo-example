use alloc::boxed::Box;

use crate::{
	driver::terminal::TTYFile,
	fs::vfs::{FileHandle, FileInode, Inode, Permission, RawStat, TimeSpec},
	process::task::CURRENT,
	syscall::errno::Errno,
};

pub struct DevTTY {
	inner: TTYFile,
}

impl DevTTY {
	pub fn new(inner: TTYFile) -> Self {
		Self { inner }
	}
}

impl Inode for DevTTY {
	fn stat(&self) -> Result<RawStat, Errno> {
		Ok(RawStat {
			perm: 0o666,
			uid: 0,
			gid: 0,
			size: 0,
			file_type: 1,
			access_time: TimeSpec::default(),
			modify_fime: TimeSpec::default(),
			change_time: TimeSpec::default(),
		})
	}

	fn chown(&self, _owner: usize, _groupp: usize) -> Result<(), Errno> {
		Err(Errno::EPERM)
	}

	fn chmod(&self, _perm: Permission) -> Result<(), Errno> {
		Err(Errno::EPERM)
	}
}

impl FileInode for DevTTY {
	fn open(&self) -> Result<Box<dyn FileHandle>, Errno> {
		let current = unsafe { CURRENT.get_mut() };

		if let Some(ref ext) = current.get_user_ext() {
			let sess = &ext.lock_relation().get_session();
			if let Ok(_) = self.inner.lock_tty().connect(sess) {
				sess.lock().set_ctty(self.inner.clone());
			}
		}

		Ok(Box::new(self.inner.clone()))
	}

	fn truncate(&self, _length: isize) -> Result<(), Errno> {
		Err(Errno::EPERM)
	}
}
