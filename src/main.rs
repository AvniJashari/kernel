#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
mod print;


#[inline(always)]
fn do_nothing() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}


#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
pub extern "C" fn boot() -> ! {
    unsafe {
        asm!(
            "la sp, __stack_top",  // carica il puntatore a stack top in sp(stack poiter)
            "j {main}",            // Jump verso il main
            main = sym main,       // definisco main a livello di assembly
            options(noreturn)      // questa funzione non fa return
        );
    }
}


unsafe extern "C" {
    static mut __bss: u8;
    static mut __bss_end: u8;
}

fn main() -> ! {
    unsafe {set_bss_to_zero();}

    println!("Hello word!");

    do_nothing();
}

unsafe fn set_bss_to_zero() -> (){
    let bss_start = &raw mut __bss;
    let bss_size = (&raw mut __bss_end as usize) - (&raw mut __bss as usize);
    core::ptr::write_bytes(bss_start, 0, bss_size);
}


use core::panic::PanicInfo;

#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi"); // Wait for an interrupt (idle loop)
        }
    }
}
