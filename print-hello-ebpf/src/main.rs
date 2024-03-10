#![no_std]
#![no_main]

use aya_bpf::{macros::uprobe, programs::ProbeContext};
use aya_log_ebpf::info;

#[uprobe]
pub fn print_hello(ctx: ProbeContext) -> u32 {
    match try_print_hello(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_print_hello(ctx: ProbeContext) -> Result<u32, u32> {
    info!(&ctx, "go function hello called ");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
