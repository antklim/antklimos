#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(antklimos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use antklimos::println;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use antklimos::memory;
    use x86_64::structures::paging::MapperAllSizes;
    use x86_64::VirtAddr;

    println!("AntklimOS, version {}", "0.0.1");
    antklimos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    antklimos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    antklimos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    antklimos::test_panic_handler(info);
}
