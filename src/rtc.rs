use core::arch::asm;

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

fn update_flag() -> bool {
    out_byte(0x70, 0x0a);
    (in_byte(0x71) & 0x80) != 0
}

fn rtc_register(register: u8) -> u8 {
    out_byte(0x70, register);
    in_byte(0x71)
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

    register_b = rtc_register(0x0b);

    if register_b & 0x04 == 0 {
        second = (second & 0x0f) + ((second / 16) * 10);
        minute = (minute & 0x0f) + ((minute / 16) * 10);
        hour = (hour & 0x0f) + (((hour & 0x70) / 16) * 10);
        day = (day & 0x0f) + ((day / 16) * 10);
        month = (month & 0x0f) + ((month / 16) * 10);
        year = (year & 0x0f) + ((year / 16) * 10);
    }
     
    if (register_b & 0x02) == 0 {
        if hour & 0x80 != 0 {
            if hour & 0x7f != 12 {
                hour = ((hour & 0x7f) +12) % 24;
            }
        } else if hour == 12 {
            hour = 0;
        }
    }
    
    (second, minute, hour, day, month, year)
}
