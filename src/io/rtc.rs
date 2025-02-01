// can be better

use core::arch::asm;

const WRITE_PORT: u16 = 0x70;
const READ_PORT: u16 = 0x71;

fn out_byte(port: u16, value: u8) {
    unsafe {
        asm!("out dx,al", in("dx") port, in("al") value);
    }
}

fn in_byte(port: u16) -> u8 {
    let mut value: u8;

    unsafe {
        asm!("in al, dx", in("dx") port, out("al") value);
    }

    value
}

fn disable_nmi() {
    out_byte(WRITE_PORT, in_byte(WRITE_PORT) | 0x80);
}

fn _enable_nmi() {
    out_byte(WRITE_PORT, in_byte(WRITE_PORT) | 0x7F);
}

fn update_flag() -> bool {
    out_byte(WRITE_PORT, 0x0A);
    (in_byte(READ_PORT) & 0x80) != 0
}

fn rtc_register(register: u8) -> u8 {
    out_byte(WRITE_PORT, register);
    in_byte(READ_PORT)
}

pub fn init_rtc() {
    disable_nmi();
    
    out_byte(WRITE_PORT, 0x8A);
    let prev = in_byte(READ_PORT);

    // set 1Hz
    out_byte(WRITE_PORT, 0x8A);
    out_byte(READ_PORT, prev & 0xF0 | 0b0110);

    out_byte(WRITE_PORT, 0x8B);
    let prev = in_byte(READ_PORT);
    out_byte(WRITE_PORT, 0x8B);
    out_byte(READ_PORT, prev | 0x40);
}

pub fn read_rtc() -> (u8, u8, u8, u8, u8, u8) {
    let (mut second, mut minute, mut hour): (u8, u8, u8);
    let (mut day, mut month, mut year): (u8, u8, u8);
    let register_b;

    while update_flag() {}

    second = rtc_register(0x00);
    minute = rtc_register(0x02);
    hour = rtc_register(0x04);
    day = rtc_register(0x07);
    month = rtc_register(0x08);
    year = rtc_register(0x09);

    register_b = rtc_register(0x0B);
    
    if register_b & 0x04 == 0 {
        second = (second & 0x0F) + ((second / 16) * 10);
        minute = (minute & 0x0F) + ((minute / 16) * 10);
        hour = (hour & 0x0F) + (((hour & 0x70) / 16) * 10);
        day = (day & 0x0F) + ((day / 16) * 10);
        month = (month & 0x0F) + ((month / 16) * 10);
        year = (year & 0x0F) + ((year / 16) * 10);
    }

    (second, minute, hour, day, month, year)
}
