#![no_std]
#![no_main]
#![allow(dead_code)]
#![feature(allocator_api)]
#![feature(maybe_uninit_uninit_array)]
#![feature(const_maybe_uninit_uninit_array)]
#![feature(asm_const)]

extern crate alloc;

mod acpi;
mod backtrace;
mod boot;
mod collection;
mod config;
mod console;
mod driver;
mod file;
mod input;
mod interrupt;
mod io;
mod mm;
mod printk;
mod process;
mod ptr;
mod scheduler;
mod smp;
mod subroutine;
mod sync;
mod syscall;
mod test;
mod user_bin;
mod util;
mod x86;

use console::CONSOLE_MANAGER;
use core::{arch::asm, panic::PanicInfo};
use process::context::yield_now;
use process::task::{Task, TASK_QUEUE};
use scheduler::work::slow_worker;
use test::{exit_qemu_with, TEST_ARRAY};

use crate::config::CONSOLE_COUNTS;

use crate::mm::alloc::page::get_available_pages;
use crate::mm::constant::{MB, PAGE_SIZE};

/// very simple panic handler.
/// that just print panic infomation and fall into infinity loop.
///
/// we should make sure no more `panic!()` from here.
#[panic_handler]
fn panic_handler_impl(info: &PanicInfo) -> ! {
	printk_panic!("{}\ncall stack (most recent call first)\n", info);

	unsafe {
		print_stacktrace!();
		CONSOLE_MANAGER
			.assume_init_mut()
			.set_foreground(CONSOLE_COUNTS - 1);
		CONSOLE_MANAGER.assume_init_mut().screen_draw();
	};

	if cfg!(ktest) {
		exit_qemu_with(1);
	}

	loop {}
}

fn run_test() -> ! {
	let tests = TEST_ARRAY.as_slice();
	let n_test = tests.len();

	for (i, test) in tests.iter().enumerate() {
		pr_info!("Test [{}/{}]: {}", i + 1, n_test, test.get_name());
		test.run();
		pr_info!("...\x1b[32mPASS\x1b[39m");
	}

	pr_info!("All test PASSED.");

	exit_qemu_with(0);
}

fn idle() -> ! {
	loop {
		yield_now();
	}
}

fn run_process() -> ! {
	let a = Task::new_kernel(show_page_stat as usize, 0).expect("OOM");
	let worker = Task::new_kernel(slow_worker as usize, 0).expect("OOM");

	// use user_bin::INIT_CODE;
	// let init = Task::new_user(INIT_CODE).expect("OOM");

	TASK_QUEUE.lock().push_back(a);
	TASK_QUEUE.lock().push_back(worker);
	// TASK_QUEUE.lock().push_back(init);

	idle();
}

extern "C" fn repeat_x(x: usize) -> ! {
	loop {
		pr_info!("FROM X={}", x);
		unsafe { asm!("hlt") }
	}
}

extern "C" fn show_page_stat(_: usize) -> ! {
	loop {
		let pages = get_available_pages();
		pr_info!("AVAILABLE PAGES: {} ({} MB)", pages, pages * PAGE_SIZE / MB);
		unsafe { asm!("hlt") }
	}
}

unsafe fn kernel_boot_alloc(bi_header: usize, magic: u32) {
	let mut bootalloc = boot::init(bi_header, magic).expect("boot information");

	let meta_page_table = mm::page::alloc_meta_page_table(&mut bootalloc);
	mm::page::init(meta_page_table);

	bootalloc.deinit();
}

#[no_mangle]
pub fn kernel_entry(bi_header: usize, magic: u32) -> ! {
	driver::vga::text_vga::init();
	driver::serial::init();

	// caution: order sensitive.
	unsafe {
		kernel_boot_alloc(bi_header, magic);
	}

	interrupt::apic::local::init().unwrap();
	interrupt::idt::init();

	mm::alloc::page::init();
	mm::alloc::phys::init();
	mm::alloc::virt::init();

	console::console_manager::init();

	acpi::init();
	interrupt::apic::io::init().unwrap();

	unsafe { x86::init() };

	driver::ps2::init().expect("failed to init PS/2");
	scheduler::work::init().expect("worker thread init");

	process::init();

	match cfg!(ktest) {
		true => run_test(),
		false => run_process(),
	};
}
