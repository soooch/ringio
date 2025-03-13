#![no_std]
#![no_main]

extern crate origin;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    atomic_dbg::eprintln!("{info}");
    origin::program::exit(1)
}

#[unsafe(no_mangle)]
fn origin_main(_argc: usize, _argv: *mut *mut u8, _envp: *mut *mut u8) -> i32 {
    atomic_dbg::eprintln!("hi");
    atomic_dbg::dbg!(ringio::four());
    //panic!("dasf");
    57
}
