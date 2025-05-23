use x86_64::{registers::control::Cr2, structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode}};
use pic8259::ChainedPics;
use lazy_static::lazy_static;
use crate::{gdt, hlt_loop, println, task::executor::WAKE_RTC_TASK};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
    Rtc = PIC_2_OFFSET
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe {
    ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
});


lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);

        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);

        idt.page_fault.set_handler_fn(page_fault_handler);

        idt[InterruptIndex::Rtc.as_usize()].set_handler_fn(rtc_interrupt_handler);
        
        idt
    };
}

pub extern "x86-interrupt" fn rtc_interrupt_handler(_stack_frame: InterruptStackFrame) {
    println!("RTC interrupt");

    WAKE_RTC_TASK.wake();

    unsafe {
        x86_64::instructions::port::PortReadOnly::<u8>::new(0x70).read();
        x86_64::instructions::port::PortReadOnly::<u8>::new(0x71).read();

    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {

    println!("EXECEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);

    hlt_loop();
}

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _statck_frame: InterruptStackFrame) 
{
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    crate::io::keyboard::add_scancode(scancode);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

extern "x86-interrupt" fn timer_interrupt_handler( 
    _statck_frame: InterruptStackFrame ) 
{
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(
                InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn breakpoint_handler(statck_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREACKPOINT\n{:#?}", statck_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    statck_frame: InterruptStackFrame, 
    _error_code: u64
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", statck_frame);
}

pub fn init_idt() {
    IDT.load();
}