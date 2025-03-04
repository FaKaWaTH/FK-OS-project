#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_project::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os_project::{
    io, println, sleep, task::{
        executor::Executor,
        Task,
    }
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use os_project::{
        allocator,
        memory::{self},
    };
    use x86_64::VirtAddr;

    println!("Running");
    os_project::init();

    println!("");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(io::keyboard::print_keypresses()));
    executor.spawn(Task::new(test_sleep()));
    //executor.spawn(Task::new(date_time()));
    executor.run();

}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    os_project::hlt_loop();
}

async fn test_sleep() {
    println!("Wait 3 sec");

    sleep::sleep(3).await;

    println!("3sec after");
}

/*
async fn date_time() {
    use core::{future::poll_fn, task::Poll};
    use os_project::task::executor::{WAKE_RTC_TASK, yield_now};
    
    let mut last_time = os_project::io::rtc::read_rtc();

    loop {
        poll_fn(|cx| {
            WAKE_RTC_TASK.register(cx.waker());
            Poll::Ready(())
        })
        .await;

        let current_time = os_project::io::rtc::read_rtc();

        if current_time != last_time {
            let (second, minute, hour, day, month, year) = current_time;
        
            println!(
                "{}:{}:{}\t\t\t{}/{}/{}",
                hour, minute, second, day, month, year
            );

            last_time = current_time;
        }
        yield_now().await;
    }
}
*/

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_project::test_panic_handler(info)
}
