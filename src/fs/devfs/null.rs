use alloc::boxed::Box;

use crate::{
	fs::vfs::{
		FileHandle, FileInode, IOFlag, Inode, Permission, Statx, StatxMode, StatxTimeStamp, Whence,
	},
	syscall::errno::Errno,
};

pub struct DevNull;

impl Inode for DevNull {
	fn stat(&self) -> Result<Statx, Errno> {
		Ok(Statx {
			mask: Statx::MASK_ALL,
			blksize: 0,
			attributes: 0,
			nlink: 0,
			uid: 0,
			gid: 0,
			mode: StatxMode::new(StatxMode::CHARDEV, 0o666),
			pad1: 0,
			ino: 0,
			size: 0,
			blocks: 0,
			attributes_mask: 0,
			atime: StatxTimeStamp::default(),
			btime: StatxTimeStamp::default(),
			ctime: StatxTimeStamp::default(),
			mtime: StatxTimeStamp::default(),
			rdev_major: 0,
			rdev_minor: 0,
			dev_major: 0,
			dev_minor: 0,
		})
	}

	fn chown(&self, _owner: usize, _group: usize) -> Result<(), Errno> {
		Err(Errno::EPERM)
	}

	fn chmod(&self, _perm: Permission) -> Result<(), Errno> {
		Err(Errno::EPERM)
	}
}

impl FileInode for DevNull {
	fn open(&self) -> Result<Box<dyn FileHandle>, Errno> {
		Ok(Box::new(DevNull))
	}

	fn truncate(&self, _length: isize) -> Result<(), Errno> {
		Ok(())
	}
}

impl FileHandle for DevNull {
	fn read(&self, _buf: &mut [u8], _flags: IOFlag) -> Result<usize, Errno> {
		Ok(0)
	}

	fn write(&self, buf: &[u8], _flags: IOFlag) -> Result<usize, Errno> {
		Ok(buf.len())
	}

	fn lseek(&self, _offset: isize, _whence: Whence) -> Result<usize, Errno> {
		Ok(0)
	}
}
