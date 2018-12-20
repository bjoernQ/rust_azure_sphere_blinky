#![feature(lang_items, start, libc)]
#![no_std]
use core::panic::PanicInfo;

extern {
    pub fn GPIO_OpenAsOutput(gpioId: i32, outputMode: i8, valueType: i8) -> i32;
    pub fn GPIO_SetValue(fd: i32, valueType: i8) -> i32;
    pub fn sleep(seconds: u32) -> ();
}


#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    const MT3620_GPIO8: i32 = 8;
    const MT3620_GPIO9: i32 = 9;
    const MT3620_GPIO10: i32 = 10;
    const MT3620_RDB_LED1_GREEN: i32 = MT3620_GPIO9;
    const MT3620_RDB_LED1_BLUE: i32 = MT3620_GPIO10;
    const MT3620_RDB_LED1_RED: i32 = MT3620_GPIO8;

    const GPIO_OUTPUT_MODE_PUSH_PULL: i8 = 0;
    const GPIO_VALUE_LOW: i8 = 0;
    const GPIO_VALUE_HIGH: i8 = 1;

    unsafe {
        let fd = GPIO_OpenAsOutput(MT3620_RDB_LED1_GREEN, GPIO_OUTPUT_MODE_PUSH_PULL, GPIO_VALUE_HIGH);
        let fd2 = GPIO_OpenAsOutput(MT3620_RDB_LED1_BLUE, GPIO_OUTPUT_MODE_PUSH_PULL, GPIO_VALUE_HIGH);
        let fd3 = GPIO_OpenAsOutput(MT3620_RDB_LED1_RED, GPIO_OUTPUT_MODE_PUSH_PULL, GPIO_VALUE_HIGH);

        loop {
            GPIO_SetValue(fd, GPIO_VALUE_LOW);
            sleep(2);

            GPIO_SetValue(fd, GPIO_VALUE_HIGH);
            sleep(2);

            GPIO_SetValue(fd2, GPIO_VALUE_LOW);
            sleep(2);

            GPIO_SetValue(fd2, GPIO_VALUE_HIGH);
            sleep(2);

            GPIO_SetValue(fd3, GPIO_VALUE_LOW);
            sleep(2);

            GPIO_SetValue(fd3, GPIO_VALUE_HIGH);
            sleep(2);

        }
    }

    0
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
