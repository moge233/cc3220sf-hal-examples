#![no_std]
#![no_main]


// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate embedded_hal;
#[macro_use(block)]
extern crate nb;


use cortex_m_rt::entry;

use cc3220sf_hal::pac;
use cc3220sf_hal::gpio::*;
use cc3220sf_hal::timer::*;
use cc3220sf_hal::prcm::mcu_init;
use embedded_hal::digital::v2::*;
use embedded_hal::prelude::*;


#[entry]
fn main() -> ! {

    mcu_init();

    let device_peripherals = pac::Peripherals::take().unwrap();
    let port1 = device_peripherals.GPIOA1.split();

    let mut red_led = port1.gpio_09.into_push_pull_output();
    let mut yel_led = port1.gpio_10.into_push_pull_output();
    let mut grn_led = port1.gpio_11.into_push_pull_output();

    // Timer operates off 80MHz core clock
    // Time     Ticks
    // --------------------
    // 1s       0x04C4B400
    // 0.5s     0x02625A00
    // 0.25s    0x01312d00
    // 0.125s   0x00989680
    // ...      ...
    let mut timer = Timer::timera0(device_peripherals.TIMERA0,
                                   Ticks(0x00989680));

    red_led.set_drive_strength(DriveStrength::Medium);
    yel_led.set_drive_strength(DriveStrength::Medium);
    grn_led.set_drive_strength(DriveStrength::Medium);

    red_led.set_high().unwrap();
    yel_led.set_high().unwrap();
    grn_led.set_high().unwrap();

    loop {
        block!(timer.wait()).unwrap();
        red_led.toggle().unwrap();
        yel_led.toggle().unwrap();
        grn_led.toggle().unwrap();
    }
}
