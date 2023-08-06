#[macro_export]
macro_rules! pr_err {
	($($args:tt)*) => {
		#[cfg(any(log_level = "error", log_level = "warn", log_level = "info", log_level = "debug"))]
		$crate::printk::__printk(
			$crate::fmt_with!(
				WITH(ln)
				WITH(bg 41)
				FMT($($args)*)
			)
		).unwrap()
	};
}

#[macro_export]
macro_rules! pr_warn {
	($($args:tt)*) => {
		#[cfg(any(log_level = "warn", log_level = "info", log_level = "debug"))]
		$crate::printk::__printk(
			$crate::fmt_with!(
				WITH(ln)
				WITH(bg 43)
				FMT($($args)*)
			)
		).unwrap()
	};
}

#[macro_export]
macro_rules! pr_info {
	($($args:tt)*) => {
		#[cfg(any(log_level = "info", log_level = "debug"))]
		$crate::printkln!($($args)*)
	};
}

#[macro_export]
macro_rules! pr_debug {
	($($args:tt)*) => {
		#[cfg(all(log_level = "debug"))]
		$crate::printkln!($($args)*);
	};
}

#[macro_export]
macro_rules! printkln {
	($($args:tt)*) => {
		$crate::printk::__printk(
			$crate::fmt_with!(
				WITH(ln)
				FMT($($args)*)
			)
		).unwrap()
	};
}

#[macro_export]
macro_rules! printk {
	($($args:tt)*) => {
		$crate::printk::__printk(
			$crate::fmt_with!(
				FMT($($args)*)
			)
		).unwrap()
	};
}

#[macro_export]
macro_rules! printk_panic {
	($($args:tt)*) => {
		unsafe {
			$crate::printk::__printk(
				$crate::fmt_with!(
					WITH(bg 41)
					FMT($($args)*)
				)
			).unwrap_unchecked()
		}
	};
}

#[macro_export]
macro_rules! fmt_with {
    (WITH(bg $color:literal)) => { concat!("\x1b[", $color, "m") };

	(END(bg $color:literal)) => { "\x1b[49m" };

    (WITH(ln)) => { "" };

    (END(ln)) => { "\n" };

    (HANDLE FMT($fmt:expr)) => { $fmt };

    (HANDLE WITH($($x:tt)+) $(WITH($($xs:tt)+))* FMT($fmt:expr)) => {
        concat!(
            $crate::fmt_with!(WITH($($x)+)),
            $crate::fmt_with!(HANDLE $(WITH($($xs)+))* FMT($fmt)),
            $crate::fmt_with!(END($($x)+))
        )
    };

	($(WITH($($xs:tt)+))* FMT($fmt:expr)) => {
        $crate::fmt_with!($(WITH($($xs)+))* FMT($fmt,))
    };

    ($(WITH($($xs:tt)+))* FMT($fmt:expr, $($args:tt)*)) => {
        core::format_args!($crate::fmt_with!(HANDLE $(WITH($($xs)+))* FMT($fmt)), $($args)*)
    };
}

use crate::{driver::serial, sync::spinlock::SpinLock, RUN_TIME};
use core::{
	fmt::{Arguments, Result, Write},
	sync::atomic::Ordering,
};

static PRINTK_LOCK: SpinLock = SpinLock::new();

pub fn __printk(arg: Arguments) -> Result {
	let result;
	if RUN_TIME.load(Ordering::Relaxed) {
		PRINTK_LOCK.lock();

		result = unsafe { serial::SERIAL_EXT_COM1.write_fmt(arg) };

		PRINTK_LOCK.unlock();
	} else {
		result = unsafe { serial::SERIAL_COM1.write_fmt(arg) };
	}

	result
}
