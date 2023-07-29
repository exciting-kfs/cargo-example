use core::mem::transmute;

#[repr(usize)]
#[derive(Clone, Copy, Debug)]
pub enum SigNum {
	HUP = 1,
	INT,
	QUIT,
	ILL,
	TRAP,
	ABRT,
	IOT,
	BUS,
	FPE,
	KILL,
	USR1,
	SEGV,
	USR2,
	PIPE,
	ALRM,
	TERM,
	STKFLT,
	CHLD,
	CONT,
	STOP,
	TSTP,
	TTIN,
	TTOU,
	URG,
	XCPU,
	XFSZ,
	VTALRM,
	PROF,
	WINCH,
	IO,
	PWR,
	SYS,
}

impl SigNum {
	pub fn from_usize(num: usize) -> Option<Self> {
		use SigNum::*;
		if HUP as usize <= num && num <= SYS as usize {
			Some(unsafe { transmute(num) })
		} else {
			None
		}
	}

	pub const fn index(&self) -> usize {
		*self as usize - 1
	}
}
