use super::context::{context_switch, InContext};
use super::exit::sys_exit;

/// Re-enable IRQ and execute thread routine
pub extern "C" fn kthread_entry(routine: extern "C" fn(usize) -> usize, arg: usize) {
	context_switch(InContext::Kernel);
	let ret = routine(arg);
	sys_exit(ret);
}
